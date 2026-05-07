//! old 버전 실전 사용 부적합.

// Reader-Writer Lock (RWLock):
// Reader는 여러 개 동시에 허용, Writer는 반드시 단독 실행

#include <stdbool.h>
#include <stdio.h>

#include "sync/examples.h"

static void spinlock_acquire(bool *lock) {
    while (__sync_lock_test_and_set(lock, true)) {
    }
}

static void spinlock_release(bool *lock) {
    __sync_lock_release(lock);
}

// Reader용 록 획득 함수 ❶
static void rwlock_read_acquire(int *rcnt, volatile int *wcnt) {
    for (;;) {
        while (*wcnt); // Writer가 있으면 대기 ❷
        __sync_fetch_and_add(rcnt, 1); // ❸
        if (*wcnt == 0) // Writer가 없으면 록 획득 ❹
            break;
        __sync_fetch_and_sub(rcnt, 1);  // Writer 있으면 Reader 취소
    }
}

// Reader용 록 반환 함수 ❺
static void rwlock_read_release(int *rcnt) {
    __sync_fetch_and_sub(rcnt, 1);
}

// Writer용 록 획득 함수 ❻
static void rwlock_write_acquire(bool *lock, volatile int *rcnt, int *wcnt) {
    __sync_fetch_and_add(wcnt, 1); // ❼ Writer 존재 표시
    while (*rcnt); // Reader가 있으면 대기
    spinlock_acquire(lock); // ❽ 다른 Writer도 상호배제
}

// Writer용 록 반환 함수 ❾
static void rwlock_write_release(bool *lock, int *wcnt) {
    spinlock_release(lock);
    __sync_fetch_and_sub(wcnt, 1); // Writer 종료 새 Reader 허용
}

// 공유 변수
static int  rcnt = 0;
static int  wcnt = 0;
static bool lock = false;

static void reader(void) { // Reader용 함수
    rwlock_read_acquire(&rcnt, &wcnt);
    rwlock_read_release(&rcnt);
}

static void writer(void) { // Writer용 함수
    rwlock_write_acquire(&lock, &rcnt, &wcnt);
    rwlock_write_release(&lock, &wcnt);
}

int sync_rwlock_spin_old_main(void) {
    reader();
    writer();
    printf("old rwlock example finished\n");
    return 0;
}
