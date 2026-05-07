#include <stdatomic.h>
#include <stdbool.h>
#include <stdio.h>

#include "sync/examples.h"

// Test and Set: 스핀락(spin lock, CPU가 계속 while 돌면서 기다림) 구현을 위한 대표적 기술
// 현재 값을 읽고 동시에 새 값으로 바꾸는 원자적 연산 
//* 락이 비어있으면 획득(true)
// 계속 while을 돌리기 때문에 cpu 낭비가 발생

// tas 개념적 코드 (atomic이 아님. CPU instruction(지침) 필요)
static bool tas(bool *p) {

    bool old = *p;

    *p = true;

    return old;
}

// ---------------------
// c 컴파일러 내장함수를 제공 (구식 API)
static bool test_and_set(volatile bool *p) {
    return __sync_lock_test_and_set(p, 1);
}

// unlock
static void tas_release(volatile bool *p) {
    return __sync_lock_release(p);
}

// ---------------------
// 가장 기본적인 spin lock 획득 코드 (보다 현대적, 컴파일러에 stdatomic.h 포함되어야 함.)
typedef struct {
    atomic_bool locked;
} spinlock_t;

static void spin_lock(spinlock_t *lock)
{
    while (atomic_exchange(&lock->locked, true)) {
    }
}

static void spin_unlock(spinlock_t *lock)
{
    atomic_store(&lock->locked, false);
}

int sync_tas_main(void) {
    volatile bool old_lock = false;
    spinlock_t lock = {
        .locked = false
    };

    printf("tas first = %d\n", tas((bool *)&old_lock));
    printf("test_and_set = %d\n", test_and_set(&old_lock));
    tas_release(&old_lock);

    spin_lock(&lock);
    printf("spin lock acquired\n");
    spin_unlock(&lock);

    return 0;
}
