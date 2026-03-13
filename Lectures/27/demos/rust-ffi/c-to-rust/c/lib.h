#ifndef C_TO_RUST_LIB_H
#define C_TO_RUST_LIB_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Point {
    int x;
    int y;
} Point;

int rust_multiply(int a, int b);
char* rust_concat_strings(const char* a, const char* b);
void rust_free_string(char* s);
char* rust_greet(const char* name);
int rust_point_sum(const Point* p);

#ifdef __cplusplus
}
#endif

#endif /* C_TO_RUST_LIB_H */
