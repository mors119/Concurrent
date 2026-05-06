//! old 버전 실전 사용 부적합.

// Reader-Writer Lock (RWLock):
// Reader는 여러 개 동시에 허용, Writer는 반드시 단독 실행

// Reader용 록 획득 함수 ❶
void rwlock_read_acquire(int *rcnt, volatile int *wcnt) {
    for (;;) {
        while (*wcnt); // Writer가 있으면 대기 ❷
        __sync_fetch_and_add(rcnt, 1); // ❸
        if (*wcnt == 0) // Writer가 없으면 록 획득 ❹
            break;
        __sync_fetch_and_sub(rcnt, 1);  // Writer 있으면 Reader 취소
    }
}

// Reader용 록 반환 함수 ❺
void rwlock_read_release(int *rcnt) {
    __sync_fetch_and_sub(rcnt, 1);
}

// Writer용 록 획득 함수 ❻
void rwlock_write_acquire(bool *lock, volatile int *rcnt, int *wcnt) {
    __sync_fetch_and_add(wcnt, 1); // ❼ Writer 존재 표시
    while (*rcnt); // Reader가 있으면 대기
    spinlock_acquire(lock); // ❽ 다른 Writer도 상호배제
}

// Writer용 록 반환 함수 ❾
void rwlock_write_release(bool *lock, int *wcnt) {
    spinlock_release(lock);
    __sync_fetch_and_sub(wcnt, 1); // Writer 종료 새 Reader 허용
}

// 공유 변수
int  rcnt = 0;
int  wcnt = 0;
bool lock = false;

void reader() { // Reader용 함수
    for (;;) {
        rwlock_read_acquire(&rcnt, &wcnt);
        // 크리티컬 섹션(읽기만)
        rwlock_read_release(&rcnt);
    }
}

void writer () { // Writer용 함수
    for (;;) {
        rwlock_write_acquire(&lock, &rcnt, &wcnt);
        // 크리티컬 섹션(읽기 및 쓰기)
        rwlock_write_release(&lock, &wcnt);
    }
}