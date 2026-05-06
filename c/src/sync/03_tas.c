#include <stdatomic.h>
#include <stdbool.h>

// Test and Set: 스핀락(spin lock, CPU가 계속 while 돌면서 기다림) 구현을 위한 대표적 기술
// 현재 값을 읽고 동시에 새 값으로 바꾸는 원자적 연산 
//* 락이 비어있으면 획득(true)
// 계속 while을 돌리기 때문에 cpu 낭비가 발생

// tas 개념적 코드 (atomic이 아님. CPU instruction(지침) 필요)
bool tas(bool *p) {

    bool old = *p;

    *p = true;

    return old;
}

// ---------------------
// c 컴파일러 내장함수를 제공 (구식 API)
bool test_and_set(volatile bool *p) {
    return __sync_lock_test_and_set(p, 1);
}

// unlock
void tas_release(volatile bool *p) {
    return __sync_lock_release(p);
}

// ---------------------
// 가장 기본적인 spin lock 획득 코드 (보다 현대적, 컴파일러에 stdatomic.h 포함되어야 함.)
typedef struct {
    atomic_bool locked;
} spinlock_t;

void spin_lock(spinlock_t *lock)
{
    while (atomic_exchange(&lock->locked, true)) {
    }
}

void spin_unlock(spinlock_t *lock)
{
    atomic_store(&lock->locked, false);
}