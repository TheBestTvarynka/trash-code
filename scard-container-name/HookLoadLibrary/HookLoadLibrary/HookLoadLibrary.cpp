#include <iostream>
#include <tchar.h>
#include "windows.h"

using namespace std;

// .dll to inject
char BAD_DLL_PATH[] = "E:\\Documents\\projects\\bad\\x64\\Debug\\bad.dll";
unsigned int BAD_DLL_PATH_LEN = sizeof(BAD_DLL_PATH) + 1;

void* get_load_library_a_fn()
{
    HMODULE h_kernel_base = GetModuleHandle(L"KernelBase");
    if (!h_kernel_base)
    {
        cout << "Can not acquire KernelBase handle" << endl;
        return nullptr;
    }

    void* loadl_library_a_fn = GetProcAddress(h_kernel_base, "LoadLibraryA");
    if (!loadl_library_a_fn)
    {
        cout << "Can not find LoadLibraryA in dll." << endl;
        return nullptr;
    }

    return loadl_library_a_fn;
}

bool inject_dll(HANDLE hProcess, void* load_library_a_fn)
{
    LPVOID remote_buffer = VirtualAllocEx(hProcess, NULL, BAD_DLL_PATH_LEN, (MEM_RESERVE | MEM_COMMIT), PAGE_EXECUTE_READWRITE);
    if (!remote_buffer)
    {
        cout << "Can not alloc virtual memory" << endl;
        return false;
    }

    bool result = WriteProcessMemory(hProcess, remote_buffer, BAD_DLL_PATH, BAD_DLL_PATH_LEN, NULL);
    if (!result)
    {
        cout << "Can not write into process memory" << endl;
        return false;
    }

    HANDLE remote_thread = CreateRemoteThread(hProcess, NULL, 0, (LPTHREAD_START_ROUTINE)load_library_a_fn, remote_buffer, 0, NULL);
    if (!remote_thread)
    {
        cout << "Can not create remote thread :(" << endl;
        return false;
    }

    return true;
}

// Creates a suspended "SignDataTmp.exe" process, injects the dll, and continues the execution.
void hook_new_mstsc()
{
    void* load_library_a_fn = get_load_library_a_fn();
    if (!load_library_a_fn)
        return;

    PROCESS_INFORMATION pi;
    ZeroMemory(&pi, sizeof(pi));

    STARTUPINFO si;
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);

    // Note: replace with your own exe path.
    LPWSTR exe_path = _tcsdup(TEXT("E:\\Documents\\Visual Studio 2022\\Projects\\SignDataTmp\\SignDataTmp\\bin\\Debug\\net6.0\\SignDataTmp.exe"));

    if (!CreateProcess(NULL, exe_path, NULL, NULL, false, CREATE_SUSPENDED, NULL, NULL, &si, &pi))
    {
        cout << "Can not create process :(" << endl;
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
        return;
    }

    if (!inject_dll(pi.hProcess, load_library_a_fn))
    {
        cout << "Can not to inject the dll :(" << endl;
		CloseHandle(pi.hProcess);
		CloseHandle(pi.hThread);
        return;
    }

    ResumeThread(pi.hThread);

    WaitForSingleObject(pi.hProcess, INFINITE);

	CloseHandle(pi.hProcess);
	CloseHandle(pi.hThread);
}

int main()
{
    cout << "Start:" << endl;

    hook_new_mstsc();

    cout << "Finish." << endl;

    return 0;
}

