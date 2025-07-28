#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "simplelib.h"

void print_str(char *buf, size_t len) {
    for (size_t i = 0; i < len; i++) {
        putchar(buf[i]);
    }
    putchar('\n');  
}

int main() {
    size_t len = 256;
    char *buffer = (char *)malloc(len);
    if (!buffer) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }

    int result = get_hostname(buffer, &len);
    if (result == 0) {
        printf("Hostname: ");
        print_str(buffer, len);
    } else {
        fprintf(stderr, "get_hostname failed with code %d.\n", result);
    }

    free(buffer);
    return 0;
}
