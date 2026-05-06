#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

// 조건 변수(condition variable): 
// 스레드가 **특정 조건이 충족될 때까지 뮤텍스(Mutex)를 해제하고 효율적으로 대기(Wait)하다가**,
// 다른 스레드로부터 신호(Signal/Broadcast)를 받아 작업을 재개하게 하는 동기화 도구
// 쉽게 말하면, 조건이 만족될 때까지 스레드를 잠재운다

// producer가 데이터 준비할 때까지 consumer를 sleep 상태로 기다리게 하는 예제
pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER; // ❶ 공유 데이터 보호 (ready, buf 보호)
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;  // ❷ 조건 변수 객체 (조건이 만족되면 깨움)

volatile bool ready = false; // ❸ 데이터 준비 여부 (아직 producer가 데이터 안 만듦)
char buf[256]; // 스레드 사이에서 데이터를 주고 받기 위한 버퍼

void* producer(void *arg) { // 데이터 생성 스레드 ❹
    printf("producer: ");
    fgets(buf, sizeof(buf), stdin); // 사용자 입력을 buf에 저장.

    pthread_mutex_lock(&mut); // 공유 데이터 수정 시작.
    ready = true; // ❺ 데이터 준비 완료

    // pthread_cond_signal() 스레드 하나만 깨움 (이 코드는 consumer가 하나라 single도 가능)
    // pthread_cond_broadcast() 대기 중인 모든 스레드 깨움
    if (pthread_cond_broadcast(&cond) != 0) { // 전체에 알림 ❻ 조건이 만족됐으므로 다 깨움
        perror("pthread_cond_broadcast"); exit(-1);
    }

    pthread_mutex_unlock(&mut);
    return NULL;
}

void* consumer(void *arg) { // 데이터 소비 스레드 ❼
    pthread_mutex_lock(&mut); // 공유 데이터 접근 시작.

    while (!ready) { // ready 변수값이 false인 경우 대기
        // 록 반환과 대기를 동시에 실행
        if (pthread_cond_wait(&cond, &mut) != 0) {  // * 핵심 ❽ 1. mutex 자동 unlock
                                                    // * 2. 스레드 sleep
                                                    // * 3. signal/broadcast 기다림
                                                    // * 4. 깨어나면 mutex 다시 lock
                                                    // * 5. 함수 복귀
            perror("pthread_cond_wait"); exit(-1);
        }
    }

    pthread_mutex_unlock(&mut); // consumer가 mutex 잡고 계속 대기하면 producer가 mutex 못 잡음 (deadlock)
    printf("consumer: %s\n", buf); // unlock 후 출력해야 critical section 최소화 됨.
    return NULL;
}

int main(int argc, char *argv[]) {
    // 스레드 생성
    pthread_t pr, cn;
    pthread_create(&pr, NULL, producer, NULL);
    pthread_create(&cn, NULL, consumer, NULL);

    // 스레드 종료 대기
    pthread_join(pr, NULL);
    pthread_join(cn, NULL);

    // 뮤텍스 객체 반환
    pthread_mutex_destroy(&mut);

    // 조건 변수 객체 반환 ❾
    if (pthread_cond_destroy(&cond) != 0) {
        perror("pthread_cond_destroy"); return -1;
    }

    return 0;
}