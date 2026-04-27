#include <stdio.h>
#include <string.h>

#include "matcher.h"
#include "common/ch01_test.h"

void matcher_run(int argc, char* argv[]) {
    const char* category = argv[1];
    const char* number = argv[2];

    if (strcmp(category, "common") == 0) {
        if (strcmp(number, "1") == 0) {
            ch01_test_run();
            return;
        }

        printf("common 카테고리의 해당 번호가 없습니다.\n");
        return;
    }

    printf("알 수 없는 category 입니다.\n");
}