#include <pthread.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#include "sync/examples.h"

// Apple pthread 구현은 일부 기능을 빼둠. 
// mutex + condition variable
#define NUM_THREADS 4

typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    int count;
    int max;
} barrier_t;

static barrier_t barrier;

static void barrier_init(barrier_t *barrier, int max) {
    pthread_mutex_init(&barrier->mutex, NULL);
    pthread_cond_init(&barrier->cond, NULL);

    barrier->count = 0;
    barrier->max = max;
}

static void barrier_wait(barrier_t *barrier) {
    pthread_mutex_lock(&barrier->mutex);

    barrier->count++;

    if (barrier->count == barrier->max) {
        // 마지막 스레드가 도착하면 모두 깨움
        pthread_cond_broadcast(&barrier->cond);
    } else {
        // 아직 전부 도착하지 않았으면 대기
        while (barrier->count < barrier->max) {
            pthread_cond_wait(
                &barrier->cond,
                &barrier->mutex
            );
        }
    }

    pthread_mutex_unlock(&barrier->mutex);
}

static void barrier_destroy(barrier_t *barrier) {
    pthread_mutex_destroy(&barrier->mutex);
    pthread_cond_destroy(&barrier->cond);
}

static void *worker(void *arg) {
    int id = (int)(intptr_t)arg;

    printf("thread %d: before barrier\n", id);

    barrier_wait(&barrier);

    printf("thread %d: after barrier\n", id);

    return NULL;
}

int sync_barrier_spin_mut_cond_mac_main(void) {
    pthread_t threads[NUM_THREADS];

    barrier_init(&barrier, NUM_THREADS);

    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(
            &threads[i],
            NULL,
            worker,
            (void *)(intptr_t)i
        );
    }

    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }

    barrier_destroy(&barrier);

    return 0;
}
