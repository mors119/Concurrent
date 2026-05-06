#include <stdatomic.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

// 아토믹 처리: 처리 도중 상태는 시스템으로 관측할 수 없으며
// 만약 처리가 실패하면 처리 전 상태로 복원된다.

// CAS(Compare And Swap): 현재 값이 내가 예상한 값과 같으면 새 값으로 바꾼다. 다르면 바꾸지 않는다.
// 세마포어(semaphore), 락프리(lock-free), 웨이트프리(wait-free)한 데이터 구조를 구현하기 위해 이용

// 원리는 cas의 동작이만 아래 코드는 두 동작이 분리되어 있어서
// ! 실제로는 사이에 다른 스레드가 끼어들 수 있음.
bool compare_and_swap_1(uint64_t *p, uint64_t val, uint64_t newval)
{
    if (*p != val) { // ❶ 값을 유지, false 리턴
        return false;
    }
    *p = newval; // ❷ 값 변경, true 리턴
    return true;
}

// gcc나 clang 등 c 컴파일러는 아토믹을 처리하기 위한 내장함수를 제공
bool compare_and_swap_2(uint64_t *p, uint64_t val, uint64_t newval)
{
    return __sync_bool_compare_and_swap(p, val, newval);
}

// 실무적으로는 다음과 같이 작성됨.
bool cas_uint64(
    atomic_uint_fast64_t *p,  // 원자적으로 접근할 공유 변수 주소
    uint64_t *expected,       // 내가 예상한 기존 값. 실패 시 현재 값으로 갱신됨
    uint64_t desired          // 성공하면 바꿀 새 값
) {
    return atomic_compare_exchange_strong(p, expected, desired);
}

int main(void)
{
    atomic_uint_fast64_t value = 10;

    uint64_t expected = 10;
    uint64_t desired = 20;

    bool ok = atomic_compare_exchange_strong(&value, &expected, desired);

    if (ok) {
        printf("CAS 성공: value = %lu\n", atomic_load(&value));
    } else {
        printf("CAS 실패: 현재 값 = %lu\n", expected);
    }

    return 0;
}