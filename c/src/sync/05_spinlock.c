#include <stdbool.h>
#include <stdatomic.h>
#include <pthread.h>
#include <stdio.h>

bool lock = false;

// TTAS 스핀락: Test-Test-And-Set. 먼저 읽기로 확인하고, 그다음 원자적 set을 시도하는 스핀락 방식
void spinlock_acquire(volatile bool *lock) { // ❶
    for (;;) { // 무한루프
        while(*lock); // ❷ 무한루프 (1. lock이 true인 동안 읽기만 하면서 대기)
        if (!test_and_set(lock)) // 2. 풀린 것 같으면 원자적으로 획득 시도
            break;              // 3. 성공하면 탈출
    }
}

void spinlock_release(bool *lock) {
    tas_release(lock);
}

void some_func() {
    for (;;) {
        spinlock_acquire(&lock); // 록 획득 ❶
        // 크리티컬 섹션 ❷
        spinlock_release(&lock); // 록 반환 ❸
    }
}

// 보다 현대적인 버전
typedef struct {
    atomic_bool locked;
} spinlock_t;

void spinlock_init(spinlock_t *lock) {
    atomic_init(&lock->locked, false);
}

void spinlock_acquire(spinlock_t *lock) {
    for (;;) {
        while (atomic_load_explicit(&lock->locked, memory_order_relaxed)) {
            // busy wait
        }

        if (!atomic_exchange_explicit(
                &lock->locked,
                true,
                memory_order_acquire
            )) {
            break;
        }
    }
}

void spinlock_release(spinlock_t *lock) {
    atomic_store_explicit(
        &lock->locked,
        false,
        memory_order_release
    );
}

// 실제 프로그램에서는 아래 사용
pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

int counter = 0;

void some_func_2() {

    pthread_mutex_lock(&mutex);

    // critical section
    counter++;

    pthread_mutex_unlock(&mutex);
}