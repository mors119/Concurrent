#include <stdatomic.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

// C11 표준 atomic API
#define NUM_THREADS 4
#define LOOP_COUNT 100000

atomic_int counter = 0;

void *worker(void *arg) {
    for (int i = 0; i < LOOP_COUNT; i++) {
        // C11 표준 atomic 증가
        atomic_fetch_add_explicit(
            &counter,
            1,
            memory_order_relaxed
        );
    }

    return NULL;
}

int main(void) {
    pthread_t threads[NUM_THREADS];

    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&threads[i], NULL, worker, NULL);
    }

    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }

    printf("counter = %d\n", atomic_load(&counter));

    return 0;
}