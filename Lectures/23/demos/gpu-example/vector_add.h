#pragma once

// Inefficient GPU version (bad parallelization)
void vector_add_gpu(int *d_a, int *d_b, int *d_c, int size);

// Smart GPU version (proper parallelization)
void vector_add_gpu_smart(int *d_a, int *d_b, int *d_c, int size);

// CPU version
void vector_add_cpu(int *a, int *b, int *c, int size);

