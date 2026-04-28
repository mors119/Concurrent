#include <pthread.h> // POSIX(Unix 계열 표준 API) 표준 스레드 라이브러리
#include <stdio.h>
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

void *thread_func(void *arg) {
    // intptr_t: Integer type capable of holding a pointer, 포인터 값을 안전하게 담을 수 있는 정수 타입
    int id = (int)(intptr_t)arg;
    for (int i = 0; i < 5; i++) {
        printf("id = %d, i = %d\n", id, i);
        sleep(1);
    }
    return "finished!";
}

int pthreads_main(int argc, char *argv[]) {
    pthread_t v[NUM_THREADS];
    // 스레드 생성
    for(int i = 0; i < NUM_THREADS; i++) {
        // pthread_create: = 스레드 생성
        // i번 스레드 생성, thread_func 실행, id 전달
        if (pthread_create(&v[i], NULL, thread_func, (void *)(intptr_t)i) != 0) {
            perror("pthread_create");
            return -1;
        }
    }

    // 스레드 종료 대기
    for (int i = 0; i < NUM_THREADS; i++) {
        char *ptr;
        // pthread_join: 스레드 종료 대기 (join으로 종료 하지 않으면 메모리 누출이 일어난다.)
        if (pthread_join(v[i], (void **)&ptr) == 0) {
            printf("msg = %s\n", ptr);
        } else {
            perror("pthread_join");
            return -1;
        }
    }
    return 0;
}