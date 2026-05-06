#include <stdatomic.h>
#include <stdbool.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <sched.h>

#define NUM 4
#define NUM_THREADS 10
#define NUM_LOOP 10000
// 내부 구조 파악용
typedef struct {
    // 현재 critical section에 들어간 스레드 수
    // atomic_int이므로 여러 스레드가 동시에 읽고/써도 원자적으로 처리됨
    atomic_int count;
} semaphore_t;

// 전역 세마포어 객체
// count = 0 → 현재 들어간 스레드가 없음
semaphore_t sema = {
    .count = 0
};

void semaphore_acquire(semaphore_t *sema) {
    for (;;) {
        // 현재 count 값을 읽음
        // relaxed는 단순히 값 확인용으로 가볍게 읽는 메모리 순서
        int current = atomic_load_explicit(
            &sema->count,
            memory_order_relaxed
        );

        // 이미 NUM개 이상 들어가 있으면 대기
        if (current >= NUM) {
            // CPU를 계속 태우지 않도록 다른 스레드에게 실행 기회를 양보
            sched_yield();
            continue;
        }

        // 들어갈 수 있을 것 같으면 count를 1 증가시키려고 시도
        int desired = current + 1;

        // CAS 시도
        // count가 아직 current와 같으면 desired로 바꿈
        // 다른 스레드가 먼저 count를 바꿨으면 실패하고 다시 반복
        if (atomic_compare_exchange_weak_explicit(
                &sema->count,
                &current,
                desired,
                memory_order_acquire,
                memory_order_relaxed
            )) {
            // CAS 성공 → 세마포어 획득 성공
            return;
        }
    }
}

void semaphore_release(semaphore_t *sema) {
    // critical section에서 나가므로 count를 1 감소
    atomic_fetch_sub_explicit(
        &sema->count,
        1,
        memory_order_release
    );
}

void *th(void *arg) {
    for (int i = 0; i < NUM_LOOP; i++) {
        // 세마포어 획득
        semaphore_acquire(&sema);

        // critical section
        // 동시에 최대 NUM개 스레드만 여기 들어올 수 있어야 함
        int current = atomic_load_explicit(
            &sema.count,
            memory_order_relaxed
        );

        // 검증용 코드
        // count가 NUM보다 크면 세마포어 구현이 깨진 것
        if (current > NUM) {
            printf("count = %d\n", current);
            exit(EXIT_FAILURE);
        }

        // 세마포어 반환
        semaphore_release(&sema);
    }

    return NULL;
}

int main(void) {
    pthread_t threads[NUM_THREADS];

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

    printf("OK!\n");

    return EXIT_SUCCESS;
}