#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

typedef struct {
    int* heap_ptr; // 힙 메모리 (공유됨)
    int thread_id;
} ThreadArg;

void* thread_stack_func(void* arg) {
    ThreadArg* t = (ThreadArg*)arg;

    int stack_val = 0; // 스택 변수 (각 스레드마다 따로 존재)

    for (int i = 0; i < 3; i++) {
        stack_val++;
        (*t->heap_ptr)++;

        printf("[Thread %d] stack=%d, heap=%d\n",
            t->thread_id,
            stack_val,
            *t->heap_ptr);

        sleep(1);
    }

    return NULL;
}

int stack_heap_main() {
    pthread_t t1, t2;

    int* heap_val = malloc(sizeof(int)); // 힙 메모리
    *heap_val = 0;

    ThreadArg arg1 = { heap_val, 1 };
    ThreadArg arg2 = { heap_val, 2 };

    pthread_create(&t1, NULL, thread_stack_func, &arg1);
    pthread_create(&t2, NULL, thread_stack_func, &arg2);

    pthread_join(t1, NULL);
    pthread_join(t2, NULL);

    printf("\n[Main] final heap value = %d\n", *heap_val);

    free(heap_val); // 힙 메모리 해제
    return 0;
}