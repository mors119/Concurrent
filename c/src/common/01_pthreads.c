#include <pthread.h> // ❶ pthreads 라이브러리
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

#include "common/pthreads.h"

/*
    스레드 생성 
        pthread_create
    스레드 종료 대기 
        pthread_join
    스레드 종료 
        pthread_exit
    스레드 ID 
        pthread_self
    동기화
        pthread_mutex_lock
        pthread_mutex_unlock
*/

#define NUM_THREADS 10 // 생성할 스레드 수

// 스레드용 함수
// void*는 아무 타입이나 가리킬 수 있는 포인터
static void *thread_func(void *arg) { // ❷ pthreads 함수는 void* 타입을 받아 void*를 반환해야한다.
    int id = (int)(intptr_t)arg; // ❸ arg가 int로 캐스팅
    for (int i = 0; i < 5; i++) { // ❹
        printf("id = %d, i = %d\n", id, i);
        sleep(1);
    }

    return "finished!"; // 반환값
}

int pthreads_main(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    pthread_t v[NUM_THREADS]; // ⑤ 스레드용 핸들러를 저장하는 배열 정의
    // 스레드 생성 ⑥
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&v[i], NULL, thread_func, (void *)(intptr_t)i) != 0) {
            perror("pthread_create");
            return -1;
        }
    }

    // 스레드 종료 대기 ⑦
    for (int i = 0; i < NUM_THREADS; i++) {
        char *ptr;
        // !! pthread_join으로 종료 대기 (수행하지 않으면 메모리 누출 되므로 반드시 join)
        if (pthread_join(v[i], (void **)&ptr) == 0) {
            printf("msg = %s\n", ptr);
        } else {
            perror("pthread_join");
            return -1;
        }
    }

    return 0;
}
