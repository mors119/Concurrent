#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

// 배리어 동기화: 모든 스레드가 특정 지점까지 올 때까지 전부 기다린다
// 모든 스레드가 barrier 도달할 때까지 대기 후, 그 다음 작업 시작
// 예를 들어, 2단계 작업은 모든 스레드가 1단계를 끝내야 시작 가능
#define NUM_THREADS 4

pthread_barrier_t barrier;

void *worker(void *arg) {
    int id = (int)(intptr_t)arg;

    printf("thread %d: before barrier\n", id);

    // 모든 스레드가 여기에 도착할 때까지 대기
    pthread_barrier_wait(&barrier);

    printf("thread %d: after barrier\n", id);

    return NULL;
}

int main(void) {
    pthread_t threads[NUM_THREADS];

    // NUM_THREADS개가 도착해야 barrier 통과
    pthread_barrier_init(&barrier, NULL, NUM_THREADS);

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

    pthread_barrier_destroy(&barrier);

    return 0;
}