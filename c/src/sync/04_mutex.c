#include <stdatomic.h>
#include <stdbool.h>
#include <stdio.h>

#include "sync/examples.h"

// Mutex(Mutual Execution): 배타 실행이라고도 함.
// 핵심은 "락 확인 + 락 획득"
// 크리티컬 섹션(동시에 하나의 스레드만 접근해야 하는 코드 영역)을 실행할 프로세스를 1개로 제한하는 동기처리

//! 잘못된 뮤텍스 구현
static bool lock = false; // 공유 변수 ❶

static bool test_and_set(bool *p) {
    return __sync_lock_test_and_set(p, true);
}

static void tas_release(bool *p) {
    __sync_lock_release(p);
}

static void bad_mutex(void) {
retry: // 레이블 문법: goto가 이동할 위치 (goto retry하면 여기로 실행)

    //! if(!lock)과 lock = true가 분리되어 있으므로 다른 스레드가 끼어들 수 있음.
    if (!lock) { // ❷
        lock = true; // 락 획득
        // 크리티컬 섹션
    } else {
        goto retry;
    }
    lock = false; // 락 반환 ❸
}

// 구버전
static void good_mutex_1(void) {
retry:
    if (!test_and_set(&lock)) { // 검사 및 록 획득
        // 크리티컬 섹션
    } else {
        goto retry;
    }
    tas_release(&lock); // 록 반환
}

// 신버전
static atomic_bool lock2 = false;

static void good_mutex_2(void) {
    while (atomic_exchange_explicit(
        &lock2,
        true,
        memory_order_acquire
    )) {
        // spin
    }

    // critical section

    atomic_store_explicit(
        &lock2,
        false,
        memory_order_release
    );
}

int sync_mutex_main(void) {
    bad_mutex();
    good_mutex_1();
    good_mutex_2();
    printf("mutex examples finished\n");
    return 0;
}
