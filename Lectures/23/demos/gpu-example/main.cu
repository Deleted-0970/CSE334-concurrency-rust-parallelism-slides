#include "vector_add.h"
#include <stdio.h>
#include <stdlib.h>
#include <cuda_runtime.h>
#include <time.h>

int main() {
    // Problem size
    int n = 1 << 24;  // 16 million elements for meaningful timing
    size_t bytes = n * sizeof(int);
    
    printf("Vector Addition Performance Comparison\n");
    printf("Vector size: %d elements (%.2f MB)\n\n", n, bytes / (1024.0f * 1024.0f));
    
    // Allocate host memory
    int *h_a = (int *)malloc(bytes);
    int *h_b = (int *)malloc(bytes);
    int *h_c_gpu = (int *)malloc(bytes);
    int *h_c_gpu_smart = (int *)malloc(bytes);
    int *h_c_cpu = (int *)malloc(bytes);
    
    // Initialize input data
    for (int i = 0; i < n; i++) {
        h_a[i] = i % 1000;
        h_b[i] = (2 * i) % 1000;
    }
    
    // ===== CPU TIMING =====
    printf("Running CPU version...\n");
    clock_t cpu_start = clock();
    vector_add_cpu(h_a, h_b, h_c_cpu, n);
    clock_t cpu_end = clock();
    double cpu_time = ((double)(cpu_end - cpu_start)) / CLOCKS_PER_SEC * 1000.0;
    printf("CPU time: %.3f ms\n\n", cpu_time);
    
    // ===== INEFFICIENT GPU TIMING =====
    printf("Running Inefficient GPU version (1 thread, no parallelism)...\n");
    
    // Allocate device memory
    int *d_a, *d_b, *d_c;
    cudaMalloc((void **)&d_a, bytes);
    cudaMalloc((void **)&d_b, bytes);
    cudaMalloc((void **)&d_c, bytes);
    
    // Copy data from host to device
    cudaMemcpy(d_a, h_a, bytes, cudaMemcpyHostToDevice);
    cudaMemcpy(d_b, h_b, bytes, cudaMemcpyHostToDevice);
    
    // Warm up GPU
    vector_add_gpu(d_a, d_b, d_c, n);
    
    // Time inefficient GPU kernel
    cudaEvent_t start, stop;
    cudaEventCreate(&start);
    cudaEventCreate(&stop);
    
    cudaEventRecord(start);
    vector_add_gpu(d_a, d_b, d_c, n);
    cudaEventRecord(stop);
    cudaEventSynchronize(stop);
    
    float gpu_time = 0;
    cudaEventElapsedTime(&gpu_time, start, stop);
    printf("Inefficient GPU time (kernel only): %.3f ms\n\n", gpu_time);
    
    // Copy result back to host
    cudaMemcpy(h_c_gpu, d_c, bytes, cudaMemcpyDeviceToHost);
    
    // ===== SMART GPU TIMING =====
    printf("Running Smart GPU version (proper parallelization)...\n");
    
    // Warm up
    vector_add_gpu_smart(d_a, d_b, d_c, n);
    
    // Time smart GPU kernel
    cudaEventRecord(start);
    vector_add_gpu_smart(d_a, d_b, d_c, n);
    cudaEventRecord(stop);
    cudaEventSynchronize(stop);
    
    float gpu_smart_time = 0;
    cudaEventElapsedTime(&gpu_smart_time, start, stop);
    printf("Smart GPU time (kernel only): %.3f ms\n", gpu_smart_time);
    
    // Copy result back to host
    cudaMemcpy(h_c_gpu_smart, d_c, bytes, cudaMemcpyDeviceToHost);
    
    // ===== VERIFICATION =====
    printf("\nVerifying results...\n");
    bool correct_inefficient = true;
    bool correct_smart = true;
    
    for (int i = 0; i < n; i++) {
        if (h_c_gpu[i] != h_c_cpu[i]) {
            printf("Mismatch (inefficient) at index %d: GPU=%d, CPU=%d\n", i, h_c_gpu[i], h_c_cpu[i]);
            correct_inefficient = false;
            break;
        }
    }
    
    for (int i = 0; i < n; i++) {
        if (h_c_gpu_smart[i] != h_c_cpu[i]) {
            printf("Mismatch (smart) at index %d: GPU=%d, CPU=%d\n", i, h_c_gpu_smart[i], h_c_cpu[i]);
            correct_smart = false;
            break;
        }
    }
    
    if (correct_inefficient && correct_smart) {
        printf("✓ All results match!\n");
    }
    
    // ===== PERFORMANCE SUMMARY =====
    printf("\n" "=" "=" "=" "=" "=" "=" "=" "=" "=" " Performance Summary " "=" "=" "=" "=" "=" "=" "=" "=" "=" "\n");
    printf("CPU time:                 %.3f ms\n", cpu_time);
    printf("Inefficient GPU time:     %.3f ms (%.2fx vs CPU)\n", gpu_time, cpu_time / gpu_time);
    printf("Smart GPU time:           %.3f ms (%.2fx vs CPU)\n", gpu_smart_time, cpu_time / gpu_smart_time);
    printf("\nSmart GPU vs Inefficient: %.2fx faster\n", gpu_time / gpu_smart_time);
    printf("Smart GPU Bandwidth:      %.2f GB/s\n", (3.0f * bytes / 1e9) / (gpu_smart_time / 1000.0f));
    printf("=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "=" "\n");
    
    // Clean up
    cudaEventDestroy(start);
    cudaEventDestroy(stop);
    cudaFree(d_a);
    cudaFree(d_b);
    cudaFree(d_c);
    free(h_a);
    free(h_b);
    free(h_c_gpu);
    free(h_c_gpu_smart);
    free(h_c_cpu);

    return 0;
}
