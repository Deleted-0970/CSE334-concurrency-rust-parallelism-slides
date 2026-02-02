#include <stdatomic.h>
#include <pthread.h>
#include <stdio.h>

atomic_int bar = 0;
void wait() {
    int old = atomic_fetch_add(&bar, 1);
    while (old != 2) {
        old = atomic_load(&bar);
    }
}

volatile int counter; // prevent compiler from moving into register
void *main0(void* args) {
    counter = 0;
    wait();
    for (int i = 0; i < 50; i++) counter++;
    return NULL;
}

void *main1(void* args) {
    wait();
    for (int i = 0; i < 50; i++) counter++;
    return NULL;
}


int main() {
    // launch main0 and main1
    pthread_t t0, t1;
    pthread_create(&t0, NULL, main0, NULL);
    pthread_create(&t1, NULL, main1, NULL);
    // wait on them to both finish
    pthread_join(t0, NULL);
    pthread_join(t1, NULL);
    // print out new counter value
    printf("Counter is: %d\n", counter);
}
