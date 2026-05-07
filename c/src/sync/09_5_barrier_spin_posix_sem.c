#include <pthread.h>
#include <semaphore.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#include "sync/examples.h"

// POSIX semaphore API

#define NUM_THREADS 10
#define MAX_CONCURRENT 4

static sem_t sem;

static void *worker(void *arg) {
    int id = (int)(intptr_t)arg;

    // 세마포어 획득
    // 동시에 최대 MAX_CONCURRENT개만 통과 가능
    sem_wait(&sem);

    printf("thread %d: entered critical section\n", id);

    // critical section

    printf("thread %d: leaving critical section\n", id);

    // 세마포어 반환
    sem_post(&sem);

    return NULL;
}

int sync_barrier_spin_posix_sem_main(void) {
    pthread_t threads[NUM_THREADS];

    // 초기 카운트 4
    // 즉 동시에 4개 스레드까지 통과 가능
    sem_init(&sem, 0, MAX_CONCURRENT);

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

    sem_destroy(&sem);

    return 0;
}
