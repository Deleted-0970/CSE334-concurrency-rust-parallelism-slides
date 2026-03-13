#include <stdio.h>
#include <stdlib.h>
#include "lib.h"

int main(void) {
    /* C calling Rust: rust_multiply */
    int product = rust_multiply(6, 7);
    printf("rust_multiply(6, 7) = %d\n", product);

    /* C calling Rust: rust_concat_strings */
    char* concat = rust_concat_strings("Hello, ", "FFI!");
    printf("rust_concat_strings(\"Hello, \", \"FFI!\") = %s\n", concat);
    rust_free_string(concat);

    /* C calling Rust: rust_greet */
    char* greeting = rust_greet("World");
    printf("rust_greet(\"World\") = %s\n", greeting);
    rust_free_string(greeting);

    /* C calling Rust: rust_point_sum (repr(C) struct) */
    Point p = { 3, 4 };
    int sum = rust_point_sum(&p);
    printf("rust_point_sum(Point{3, 4}) = %d\n", sum);

    printf("\nAll C->Rust FFI demos completed.\n");
    return 0;
}
