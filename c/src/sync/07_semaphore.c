// mutex → 한 번에 1개만 허용 semaphore → 한 번에 N개까지 허용
// semaphore:
// 동시에 최대 NUM개의 스레드가 critical section에 들어갈 수 있도록 제한

#define NUM 4

void semaphore_acquire(volatile int *cnt) {

    for (;;) {

        // 현재 사용 중인 스레드 수가 NUM 이상이면 대기
        while (*cnt >= NUM);

        // 일단 내가 들어간다고 count 증가
        // 원자적으로 증가 수행
        __sync_fetch_and_add(cnt, 1);

        // 증가 후에도 NUM 이하라면 획득 성공
        // (다른 스레드와 경쟁 중이었을 수 있음)
        if (*cnt <= NUM)
            break;

        // NUM 초과라면 내가 늦게 들어온 것
        // 증가했던 count를 다시 감소시키고 재시도
        __sync_fetch_and_sub(cnt, 1);
    }
}

void semaphore_release(int *cnt) {

    // critical section에서 나가므로 count 감소
    __sync_fetch_and_sub(cnt, 1);
}