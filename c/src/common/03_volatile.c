#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

#include "common/volatile.h"

// 메모리 값이 0이 아닐 때까지 대기

// 컴파일러 최적화 때문에
// while문이 "영원히 끝나지 않을 수 있음"
static void wait_while_0(int *p) {
    while (*p == 0) {}
}
// 또는 컴파일러 최적화로 컴파일러가 while (*p == 0) {}를
// if (*p == 0) { while (1) {}}로 바꿀 수도 있다.

// volatile은 "이 값은 외부에서 언제든 바뀔 수 있으니까 
// 매번 진짜 메모리에서 다시 읽어라"라는 메시지를 주는 것이다. 
// 컴파일러가 함부로 레지스터 캐싱 / 최적화 / 루프 제거를 하지 못하게 하는 것이다.
static void wait_while_1(volatile int *p) {
    while (*p == 0) {}
}

static volatile int ready = 0;

static void *set_ready(void *arg) {
    (void)arg;

    sleep(1);
    ready = 1;
    return NULL;
}

int volatile_main(void) {
    pthread_t thread;
    int local = 1;

    wait_while_0(&local);

    if (pthread_create(&thread, NULL, set_ready, NULL) != 0) {
        perror("pthread_create");
        return -1;
    }

    wait_while_1(&ready);

    if (pthread_join(thread, NULL) != 0) {
        perror("pthread_join");
        return -1;
    }

    printf("volatile wait finished\n");
    return 0;
}
