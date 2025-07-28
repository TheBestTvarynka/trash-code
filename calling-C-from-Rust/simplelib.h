#include <stddef.h>

#define SIMPLELIB_SUCCESS 0
#define SIMPLELIB_ERROR_NULL_POINTER 1
#define SIMPLELIB_ERROR_INVALID_LENGTH 2

int get_hostname(char *name, size_t *length);
