#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

#include "sync/examples.h"

// Read가 대부분인 경우에는 Mutex보다 빠르다.

static pthread_rwlock_t rwlock = PTHREAD_RWLOCK_INITIALIZER; // ❶ 공용 변수 초기화 (pthread_rwlock_init 함수로도 가능)

static int shared_data = 0; // 공유 데이터

static void* reader(void *arg) { // Reader용 함수 ❷
    (void)arg;
    if (pthread_rwlock_rdlock(&rwlock) != 0) {
        perror("pthread_rwlock_rdlock"); exit(-1);
    }

    // 여러 Reader 동시 가능
    printf(
        "read: %d\n",
        shared_data
    );
    
    if (pthread_rwlock_unlock(&rwlock) != 0) {
        perror("pthread_rwlock_unlock"); exit(-1);
    }

    return NULL;
}

static void* writer(void *arg) { // Writer용 함수 ❸
    (void)arg;
    // write lock 획득
    // 단독 접근
    if (pthread_rwlock_wrlock(&rwlock) != 0) {
        perror("pthread_rwlock_wrlock"); exit(-1);
    }

    shared_data++;

    printf(
        "write: %d\n",
        shared_data
    );

    if (pthread_rwlock_unlock(&rwlock) != 0) {
        perror("pthread_rwlock_unlock"); exit(-1);
    }

    return NULL;
}

int sync_rwlock_spin_pthreads_main(void) {
    // 스레드 생성
    pthread_t rd, wr;
    pthread_create(&rd, NULL, reader, NULL);
    pthread_create(&wr, NULL, writer, NULL);

    // 스레드 종료 대기
    pthread_join(rd, NULL);
    pthread_join(wr, NULL);

    // RW록 옵션 반환(해제)❹
    if (pthread_rwlock_destroy(&rwlock) != 0) {
        perror("pthread_rwlock_destroy"); return -1;
    }

    return 0;
}
