#include <pthread.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

#include "common/detached_thread.h"

// 스레드용 함수
static void *thread_func(void *arg) {
    (void)arg;

    for (int i = 0; i < 5; i++) {
        printf("i = %d\n", i);
        sleep(1);
    }
    return NULL;
}

// attr을 이용하지 않고 pthread_detach를 이용해서 detach 스레드 만들기
static void *thread_func_2(void *arg) {
    pthread_detach(pthread_self()); // 이런식으로 디태치 스레드화 할 수 있다.
    int id = (int)(intptr_t)arg;
    
    for (int i = 0; i < 5; i++) { 
        printf("id = %d, i = %d\n", id, i);
        sleep(1);
    }

    return NULL;
}



int detached_thread_main(int argc, char *argv[]) {
    (void)argc;
    (void)argv;

    // 어트리뷰트 초기화 ❶ 스레드 생성 옵션 설정용 객체
    pthread_attr_t attr;
    if (pthread_attr_init(&attr) != 0) {
        perror("pthread_attr_init");
        return -1;
    }

    // 디태치 스레드로 설정 ❷
    // default(일반) 스레드: join 필요 -> pthread_join 해야 메모리 정리됨
    // detached 스레드: join 불가능 -> 자동으로 종료 및 메모리 해제
    if (pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED) != 0) {
        perror("pthread_attr_setdetachstate");
        return -1;
    }

    // 어트리뷰터를 지정해 스레드 생성
    pthread_t th;
    if (pthread_create(&th, &attr, thread_func, NULL) != 0) {
        perror("pthread_join");
        return -1;
    }

    // 어트리뷰트 파기 (스레드에 영향 없음.): 설정 객체 메모리 해제
    if (pthread_attr_destroy(&attr) != 0) {
        perror("pthread_attr_destroy");
        return -1;
    }

    pthread_t th2;
    if (pthread_create(&th2, NULL, thread_func_2, (void *)(intptr_t)2) != 0) {
        perror("pthread_create");
        return -1;
    }

    sleep(7); // main 스레드가 빨리 종료되면 -> 프로세스 스레드도 같이 종료

    return 0;
}
