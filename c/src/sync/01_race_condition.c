#include <pthread.h>
#include <stdio.h>

#include "sync/examples.h"

static int counter = 0;

// race condition: 
// 프로세스가 동시에 공유 자원에 접근함에 따라 나타나는 예상치 못한 이상이나 상태
static void* worker(void* arg) {
    (void)arg;

    for (int i = 0; i < 1000000; i++) {
        counter++;
    }

    return NULL;
}

int sync_race_condition_main(void) {

    pthread_t t1;
    pthread_t t2;

    pthread_create(&t1, NULL, worker, NULL);
    pthread_create(&t2, NULL, worker, NULL);

    pthread_join(t1, NULL);
    pthread_join(t2, NULL);

    // 2000000을 예상하지만 실행할 때마다 값이 달라짐.
    printf("counter = %d\n", counter);

    return 0;
}
