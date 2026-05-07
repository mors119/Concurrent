#include <stdio.h>
#include "matcher.h"

// make
// make run
// make run ARGS="common 1"
// make clean


int main(int argc, char* argv[]) {
    if (argc < 3) {
        printf("사용법: make run ARGS=\"<category> <number>\"\n");
        printf("예시: make run ARGS=\"common 1\"\n");
        printf("예시: make run ARGS=\"sync 7-1\"\n");
        return 1;
    }

    matcher_run(argc, argv);

    return 0;
}
