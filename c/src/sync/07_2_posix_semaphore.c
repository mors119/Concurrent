#include <pthread.h>
#include <semaphore.h>
#include <stdio.h>
#include <stdlib.h>

#define NUM 4
#define NUM_THREADS 10
#define NUM_LOOP 10000

// 실제 활용

// POSIX 세마포어 객체
sem_t sem;

void *th(void *arg) {
    for (int i = 0; i < NUM_LOOP; i++) {
        // 세마포어 획득
        // 내부 카운트가 0보다 크면 1 감소하고 통과
        // 내부 카운트가 0이면 대기
        sem_wait(&sem);

        // critical section
        // 동시에 최대 NUM개 스레드만 들어올 수 있음

        // 세마포어 반환
        // 내부 카운트를 1 증가시키고 대기 중인 스레드가 있으면 깨움
        sem_post(&sem);
    }

    return NULL;
}

int main(void) {
    pthread_t threads[NUM_THREADS];

    // 세마포어 초기화
    // 두 번째 인자 0: 현재 프로세스 내부 스레드끼리 공유
    // 세 번째 인자 NUM: 초기 사용 가능 개수
    if (sem_init(&sem, 0, NUM) != 0) {
        perror("sem_init");
        return EXIT_FAILURE;
    }

    // 스레드 10개 생성
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&threads[i], NULL, th, NULL) != 0) {
            perror("pthread_create");
            return EXIT_FAILURE;
        }
    }

    // 모든 스레드가 끝날 때까지 대기
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            perror("pthread_join");
            return EXIT_FAILURE;
        }
    }

    // 세마포어 자원 정리
    sem_destroy(&sem);

    printf("OK!\n");
    return EXIT_SUCCESS;
}