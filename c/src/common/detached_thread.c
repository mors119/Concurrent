#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include "common/detached_thread.h"

void *thread_attr_func(void *arg) {
    for (int i = 0; i < 5; i++) {
        printf("i = %d\n", i);
        sleep(1);
    }
    return NULL;
}

int detached_thread_main(int argc, char *argv[]) {
  // 어프리뷰트 초기화 (스레드 생성 옵션 설정용 객체)
  pthread_attr_t attr;
  if(pthread_attr_init(&attr) != 0) {
    perror("pthread_attr_init");
    return -1;
  }

  // 디태치 스레드로 설정
  // default(일반) 스레드: join 필요 -> pthread_join 해야 메모리 정리됨
  // detached 스레드: join 불가능 -> 자동으로 종료 및 메모리 해제
  if(pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED) != 0) {
    perror("pthread_attr_setdetachstate");
    return -1;
  }

  // 어트리뷰터를 지정해 스레드 생성
  pthread_t th;
  if(pthread_create(&th, &attr, thread_attr_func, NULL) != 0) {
    perror("pthread_create");
    return -1;
  }

  // 어트리뷰터 파기 (스레드에 영향 없음.): 설정 객체 메모리 해제
  if(pthread_attr_destroy(&attr) != 0) {
    perror("pthread_attr_destroy");
    return -1;
  }

  sleep(7); // main 스레드가 빨리 종료되면 -> 프로세스 스레드도 같이 종료
  
  return 0;
}
