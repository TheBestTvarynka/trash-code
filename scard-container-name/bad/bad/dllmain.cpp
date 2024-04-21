// dllmain.cpp : Defines the entry point for the DLL application.
#include "pch.h"
#include <iostream>
#include <fstream>
#include "windows.h"
#include "MinHook.h"

#define MSRDPEX_MAX_PATH 1024

// Do not forget to set your own paths.
char LOG_FILE_PATH[] = "E:\\Documents\\messages.txt";
char WINSCARD_DLL_REPLACEMENT_PATH[] = "E:\\Documents\\projects\\sspi-rs\\target\\debug\\sspi.dll";

// Yes, it's very stupid logging but it's perfect for debugging util like this one.
void log_message(const char* message)
{
    std::ofstream logfile;
    logfile.open(LOG_FILE_PATH, std::ios_base::app);
    logfile << message << std::endl;
    logfile.close();
}

// The code below are copied from the MsRdpEx report and a little bit modified.
// The purpose of the code below is to overcome the delayed `winscard.dll` loading.
// 
// More details you can read here:
// * https://github.com/Devolutions/MsRdpEx/pull/84.
// * https://github.com/Devolutions/MsRdpEx.
int MsRdpEx_ConvertFromUnicode(UINT CodePage, DWORD dwFlags, LPCWSTR lpWideCharStr, int cchWideChar,
    LPSTR* lpMultiByteStr, int cbMultiByte, LPCSTR lpDefaultChar,
    LPBOOL lpUsedDefaultChar)
{
    int status;
    BOOL allocate = FALSE;

    if (!lpWideCharStr)
        return 0;

    if (!lpMultiByteStr)
        return 0;

    if (cchWideChar == -1)
        cchWideChar = (int)(wcslen(lpWideCharStr) + 1);

    if (cbMultiByte == 0)
    {
        cbMultiByte =
            WideCharToMultiByte(CodePage, dwFlags, lpWideCharStr, cchWideChar, NULL, 0, NULL, NULL);
        allocate = TRUE;
    }
    else if (!(*lpMultiByteStr))
        allocate = TRUE;

    if (cbMultiByte < 1)
        return 0;

    if (allocate)
    {
        *lpMultiByteStr = (LPSTR)calloc(1, cbMultiByte + 1);

        if (!(*lpMultiByteStr))
        {
            return 0;
        }
    }

    status = WideCharToMultiByte(CodePage, dwFlags, lpWideCharStr, cchWideChar, *lpMultiByteStr,
        cbMultiByte, lpDefaultChar, lpUsedDefaultChar);

    if ((status != cbMultiByte) && allocate)
    {
        status = 0;
    }

    if ((status <= 0) && allocate)
    {
        free(*lpMultiByteStr);
        *lpMultiByteStr = NULL;
    }

    return status;
}

int MsRdpEx_ConvertToUnicode(UINT CodePage, DWORD dwFlags, LPCSTR lpMultiByteStr, int cbMultiByte,
    LPWSTR* lpWideCharStr, int cchWideChar)
{
    int status;
    BOOL allocate = FALSE;

    if (!lpMultiByteStr)
        return 0;

    if (!lpWideCharStr)
        return 0;

    if (cbMultiByte == -1)
    {
        size_t len = strnlen(lpMultiByteStr, INT_MAX);
        if (len >= INT_MAX)
            return 0;
        cbMultiByte = (int)(len + 1);
    }

    if (cchWideChar == 0)
    {
        cchWideChar = MultiByteToWideChar(CodePage, dwFlags, lpMultiByteStr, cbMultiByte, NULL, 0);
        allocate = TRUE;
    }
    else if (!(*lpWideCharStr))
        allocate = TRUE;

    if (cchWideChar < 1)
        return 0;

    if (allocate)
    {
        *lpWideCharStr = (LPWSTR)calloc(cchWideChar + 1, sizeof(WCHAR));

        if (!(*lpWideCharStr))
        {
            return 0;
        }
    }

    status = MultiByteToWideChar(CodePage, dwFlags, lpMultiByteStr, cbMultiByte, *lpWideCharStr, cchWideChar);

    if (status != cchWideChar)
    {
        if (allocate)
        {
            free(*lpWideCharStr);
            *lpWideCharStr = NULL;
            status = 0;
        }
    }

    return status;
}

bool MsRdpEx_IStringEndsWithW(const WCHAR* str, const WCHAR* val)
{
    int strLen;
    int valLen;
    const WCHAR* p;

    if (!str || !val)
        return false;

    strLen = wcslen(str);
    valLen = wcslen(val);

    if ((strLen < 1) || (valLen < 1))
        return false;

    if (valLen > strLen)
        return false;

    p = &str[strLen - valLen];

    if (!_wcsicmp(p, val))
        return true;

    return false;
}

typedef struct _UNICODE_STRING
{
    USHORT Length;
    USHORT MaximumLength;
    PWSTR  Buffer;
} UNICODE_STRING, * PUNICODE_STRING;

typedef struct _OBJECT_ATTRIBUTES
{
    ULONG Length;
    HANDLE RootDirectory;
    PUNICODE_STRING ObjectName;
    ULONG Attributes;
    PVOID SecurityDescriptor;
    PVOID SecurityQualityOfService;
} OBJECT_ATTRIBUTES, * POBJECT_ATTRIBUTES;

typedef struct _IO_STATUS_BLOCK {
    union {
        LONG Status;
        PVOID Pointer;
    };
    ULONG_PTR Information;
} IO_STATUS_BLOCK, * PIO_STATUS_BLOCK;

typedef LONG (NTAPI* Func_NtOpenFile)(PHANDLE, ACCESS_MASK,
    POBJECT_ATTRIBUTES, PIO_STATUS_BLOCK,
    ULONG, ULONG);

Func_NtOpenFile g_NtOpenFile_original = NULL;
Func_NtOpenFile f_NtOpenFile = NULL;

HMODULE g_hNtDll = NULL;

LONG NtOpenFile_replacement(PHANDLE FileHandle, ACCESS_MASK DesiredAccess,
    POBJECT_ATTRIBUTES ObjectAttributes, PIO_STATUS_BLOCK IoStatusBlock,
    ULONG ShareAccess, ULONG OpenOptions)
{
    log_message("Hook: NtOpenFile");
    LONG ntstatus;
    char* pObjectNameA = NULL;
    bool interceptedCall = false;

    if (ObjectAttributes && ObjectAttributes->ObjectName && ObjectAttributes->ObjectName->Buffer &&
        MsRdpEx_IStringEndsWithW(ObjectAttributes->ObjectName->Buffer, L"WinSCard.dll"))
    {
        MsRdpEx_ConvertFromUnicode(CP_UTF8, 0, ObjectAttributes->ObjectName->Buffer,
            ObjectAttributes->ObjectName->Length, &pObjectNameA, 0, NULL, NULL);

        char NewFilePathA[MSRDPEX_MAX_PATH];
        WCHAR* NewFilePathW = NULL;
        OBJECT_ATTRIBUTES NewObjectAttributes;
        UNICODE_STRING NewObjectName;

        sprintf_s(NewFilePathA, MSRDPEX_MAX_PATH, "\\??\\%s", WINSCARD_DLL_REPLACEMENT_PATH);
        MsRdpEx_ConvertToUnicode(CP_UTF8, 0, NewFilePathA, -1, &NewFilePathW, 0);
        CopyMemory(&NewObjectAttributes, ObjectAttributes, sizeof(OBJECT_ATTRIBUTES));
        NewObjectName.Buffer = NewFilePathW;
        NewObjectName.Length = wcslen(NewFilePathW) * 2;
        NewObjectName.MaximumLength = NewObjectAttributes.ObjectName->Length;
        NewObjectAttributes.ObjectName = &NewObjectName;

        log_message("NtOpenFile: replacing");
        log_message(pObjectNameA);
        log_message(NewFilePathA);
        ntstatus = g_NtOpenFile_original(FileHandle, DesiredAccess, &NewObjectAttributes, IoStatusBlock, ShareAccess, OpenOptions);
        interceptedCall = true;

        free(NewFilePathW);
    }

    if (!interceptedCall)
    {
        ntstatus = g_NtOpenFile_original(FileHandle, DesiredAccess,
            ObjectAttributes, IoStatusBlock,
            ShareAccess, OpenOptions);
    }

    free(pObjectNameA);

    return ntstatus;
}
// End of the copied code.

bool install_load_library_hook()
{
    if (MH_Initialize() != MH_OK)
    {
		log_message("Can not initialize MinHook :(");
        return false;
    }

    g_hNtDll = GetModuleHandleA("ntdll.dll");
    if (!g_hNtDll)
    {
        log_message("ntdll module handle is null :(");
        return false;
    }
    else
    {
        log_message("ntdll :)");
    }
    f_NtOpenFile = (Func_NtOpenFile)GetProcAddress(g_hNtDll, "NtOpenFile");
    if (!f_NtOpenFile)
    {
        log_message("NtOpenFile is null :(");
        return false;
    }
    else
    {
        log_message("NtOpenFile :)");
    }

    MH_STATUS status = MH_CreateHook(f_NtOpenFile, &NtOpenFile_replacement, (LPVOID*)(&g_NtOpenFile_original));
    if (status != MH_OK)
    {
		log_message("MH_CreateHook NtOpenFile failed :(");
        char buff[255];
        _itoa_s(status, buff, 10);
        log_message(buff);
        return false;
    }
    if (MH_EnableHook(MH_ALL_HOOKS) != MH_OK)
    {
		log_message("MH_EnableHook NtOpenFile failed :(");
        return false;
    }

    return true;
}

BOOL APIENTRY DllMain( HMODULE hModule,
                       DWORD  ul_reason_for_call,
                       LPVOID lpReserved
                     )
{
    // We do not care about detach because it's a test project for debugging.
    switch (ul_reason_for_call)
    {
    case DLL_PROCESS_ATTACH:
		log_message("dll main: DLL_PROCESS_ATTACH ;");
        install_load_library_hook();
        break;
    case DLL_THREAD_ATTACH:
		log_message("dll main: DLL_THREAD_ATTACH ;");
        break;
    case DLL_THREAD_DETACH:
		log_message("dll main: DLL_THREAD_DETACH ;");
        break;
    case DLL_PROCESS_DETACH:
		log_message("dll main: DLL_PROCESS_DETACH ;");
        break;
    }

    return TRUE;
}

