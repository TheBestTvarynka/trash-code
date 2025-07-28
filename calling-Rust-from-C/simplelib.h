#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define SIMPLELIB_SUCCESS 0

#define SIMPLELIB_ERROR_NULL_POINTER 1

#define SIMPLELIB_ERROR_INVALID_LENGTH 2

/**
 * Writes the current machine hostname into the provided buffer.
 *
 * The resulting hostname length is written into the `length` parameter **without the NULL terminator char**.
 *
 * # Safety
 *
 * * The `name` pointer must not be null and point to the properly initialized and aligned memory.
 * * The `length` pointer must not be null and contain the `name` buffer size.
 */
int get_hostname(char *name,
                 size_t *length);
