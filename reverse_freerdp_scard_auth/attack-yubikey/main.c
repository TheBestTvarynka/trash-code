#include <stdio.h>
#include <dlfcn.h>

#include "pkcs11.h"

#define MAX_SLOTS 64

int tbt() {
    printf("TBT: TheBestTvarynka: 3.\n");

    char* pkcs11_module = "libykcs11.so.2.5.2";
    CK_ULONG nslots = 0;
    CK_SLOT_ID slots[MAX_SLOTS];
    CK_RV rv = 0;
    void* module = NULL;
    CK_FUNCTION_LIST_PTR p11;
    
    typedef CK_RV (*c_get_function_list_t)(CK_FUNCTION_LIST_PTR_PTR);
    c_get_function_list_t c_get_function_list = NULL;

    module = dlopen(pkcs11_module, RTLD_LOCAL | RTLD_LAZY);
    if (!module) {
        printf("TBT: can not load pkcs11 module: %p. `%s`: %s :(\n", module, pkcs11_module, dlerror());
        return 2;
    } else {
        printf("TBT: pkcs11 module successfully loaded!\n");
    }

    c_get_function_list = (c_get_function_list_t)dlsym(module, "C_GetFunctionList");
    if (!c_get_function_list)
    {
        printf("TBT: can not find `C_GetFunctionList` symbol: %p :(\n", c_get_function_list);
        return 2;
    } else {
        printf("TBT: successfully found C_GetFunctionList!\n");
    }

    rv = c_get_function_list(&p11);
    if (rv != CKR_OK) {
        printf("TBT: C_GetFunctionList: failed! :(\n");
        return 2;
    } else {
        printf("TBT: C_GetFunctionList: succeeded;\n");
    }

    rv = p11->C_Initialize(NULL);
    if (rv != CKR_OK) {
        printf("TBT: C_Initialize: failed! :(\n");
        return 2;
    } else {
        printf("TBT: C_Initialize: succeeded;\n");
    }

    rv = p11->C_GetSlotList(CK_TRUE, NULL, &nslots);
    if (rv != CKR_OK) {
        printf("TBT: C_GetSlotList(true, null, ptr): failed! :(\n");
        return 2;
    } else {
        printf("TBT: C_GetSlotList(true, null, ptr): succeeded;\n");
        printf("TBT: C_GetSlotList(true, null, ptr): we have %d | %ld | %lu slots;\n", nslots, nslots, nslots);
    }

    rv = p11->C_GetSlotList(CK_TRUE, slots, &nslots);
    if (rv != CKR_OK) {
        printf("TBT: C_GetSlotList(true, ptr, ptr): failed! :(\n");
        return 2;
    } else {
        printf("TBT: C_GetSlotList(true, ptr, ptr): succeeded;\n");
        printf("TBT: C_GetSlotList(true, null, ptr): we have %d | %ld | %lu slots;\n", nslots, nslots, nslots);
    }

    return 0;
}

int main() {
    return tbt();
}
