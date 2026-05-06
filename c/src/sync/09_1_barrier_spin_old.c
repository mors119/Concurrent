#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

// 구버전 gcc/clang 등 컴파일러
#define NUM_THREADS 4
#define LOOP_COUNT 100000

int counter = 0;

void *worker(void *arg) {
    for (int i = 0; i < LOOP_COUNT; i++) {
        // counter 값을 원자적으로 1 증가
        __sync_fetch_and_add(&counter, 1);
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

    printf("counter = %d\n", counter);

    return 0;
}