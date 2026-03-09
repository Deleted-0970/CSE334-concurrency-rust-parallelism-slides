#include "vector_add.h"
#include <cuda_runtime.h>
#include <stdio.h>

// INEFFICIENT GPU kernel - demonstrates bad GPU performance
// Each thread redundantly processes ALL elements (no parallelism!)
__global__ void vector_add_kernel(int *d_a, int *d_b, int *d_c, int size) {
    for (int i = 0; i < size; i++) {
        d_c[i] = d_a[i] + d_b[i];
    }
}

// SMART GPU kernel - proper parallelization
// Each thread processes ONE element
__global__ void vector_add_kernel_smart(int *d_a, int *d_b, int *d_c, int size) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < size) {
        d_c[idx] = d_a[idx] + d_b[idx];
    }
}

// Inefficient GPU wrapper function
void vector_add_gpu(int *d_a, int *d_b, int *d_c, int size) {
    // Launch with just 1 block, 1 thread since the kernel does everything
    vector_add_kernel<<<1, 1>>>(d_a, d_b, d_c, size);
    cudaDeviceSynchronize();
}

// Smart GPU wrapper function
void vector_add_gpu_smart(int *d_a, int *d_b, int *d_c, int size) {
    int blockSize = 256;
    int gridSize = (size + blockSize - 1) / blockSize;
    vector_add_kernel_smart<<<gridSize, blockSize>>>(d_a, d_b, d_c, size);
    cudaDeviceSynchronize();
}

// CPU implementation for comparison
void vector_add_cpu(int *a, int *b, int *c, int size) {
    for (int i = 0; i < size; i++) {
        c[i] = a[i] + b[i];
    }
}
