#include <stdlib.h>
#include <string.h>
#include <stdio.h>

/* Functions called by Rust */

int c_add(int a, int b) {
    return a + b;
}

/* Returns malloc'd string; caller must free. */
char* c_greet(const char* name) {
    size_t len = strlen("Hello from C, ") + strlen(name) + 2; /* +2 for "!" and NUL */
    char* buf = (char*)malloc(len);
    if (buf) {
        snprintf(buf, len, "Hello from C, %s!", name);
    }
    return buf;
}
