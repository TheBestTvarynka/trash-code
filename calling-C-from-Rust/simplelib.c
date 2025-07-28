#include "simplelib.h"
#include <stdio.h>

const char *const HOSTNAME = "thebesttvarynka";

int get_hostname(char *name, size_t *length) {
    if (name == NULL || length == NULL) {
        return SIMPLELIB_ERROR_NULL_POINTER;
    }

    size_t hostname_len = strlen(HOSTNAME);

    if (hostname_len > *length) {
        return SIMPLELIB_ERROR_INVALID_LENGTH;
    }

    memcpy(name, HOSTNAME, hostname_len);
    *length = hostname_len;

    return SIMPLELIB_SUCCESS;
}