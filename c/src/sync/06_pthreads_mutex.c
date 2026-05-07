#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

#include "sync/examples.h"

static pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER; // ❶ 전역 mutex 객체 생성

static void* some_func(void *arg) { // 스레드용 함수
    (void)arg;
    if (pthread_mutex_lock(&mut) != 0) { // ❷ mutex 잠그기 (이미 mutex를 잡고 있으면, 현재 스레드는 여기서 대기)
        perror("pthread_mutex_lock"); exit(-1);
    }

    // 크리티컬 섹션

    if (pthread_mutex_unlock(&mut) != 0) { // ❸ mutex 풀기 (critical section 사용 종료)
        perror("pthread_mutex_unlock"); exit(-1);
    }

    return NULL;
}

int sync_pthreads_mutex_main(void) {
    // 스레드 생성
    pthread_t th1, th2;
    if (
        pthread_create(
                &th1,       // 생성된 스레드 ID를 저장할 곳
                NULL,       // 스레드 속성. NULL이면 기본값
                some_func,  // 스레드가 실행할 함수
                NULL        // 스레드 함수에 전달할 인자
            ) != 0) {
        perror("pthread_create"); return -1;
    }

    if (pthread_create(&th2, NULL, some_func, NULL) != 0) {
        perror("pthread_create"); return -1;
    }

    // 스레드 종료 대기
    if (pthread_join(th1, NULL) != 0) {
        perror("pthread_join"); return -1;
    }

    if (pthread_join(th2, NULL) != 0) {
        perror("pthread_join"); return -1;
    }

    // 뮤텍스 객체 반환
    if (pthread_mutex_destroy(&mut) != 0) { // ❹ mutex 객체를 정리
        perror("pthread_mutex_destroy"); return -1;
    }

    return 0;
}
