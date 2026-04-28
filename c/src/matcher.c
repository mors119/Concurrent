#include <stdio.h>
#include <string.h>

#include "matcher.h"
#include "common/pthreads.h"
#include "common/detached_thread.h"
#include "common/stack_heap.h"

void matcher_run(int argc, char* argv[]) {
    const char* category = argv[1];
    const char* number = argv[2];

    if (strcmp(category, "common") == 0) {
        if (strcmp(number, "1") == 0) {
            int result = pthreads_main(argc, argv);
            if (result != 0) {
                printf("pthreads_main failed\n");
            }
            return;
        } else if(strcmp(number, "2") == 0) {
            int result = detached_thread_main(argc, argv);
            if (result != 0) {
                printf("detached_thread_main failed\n");
            }
            return;
        } else if(strcmp(number, "3") == 0) {
            int result = stack_heap_main();
            if (result != 0) {
                printf("stack_heap_main failed\n");
            }
            return;
        }

        printf("common 카테고리의 해당 번호가 없습니다.\n");
        return;
    }

    printf("알 수 없는 category 입니다.\n");
}