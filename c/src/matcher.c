#include <stdio.h>
#include <string.h>

#include "matcher.h"
#include "common/pthreads.h"
#include "common/detached_thread.h"
#include "common/stack_heap.h"
#include "common/volatile.h"
#include "sync/examples.h"

static void run_result(const char *name, int result) {
    if (result != 0) {
        printf("%s failed\n", name);
    }
}

void matcher_run(int argc, char *argv[]) {
    const char *category = argv[1];
    const char *number = argv[2];

    if (strcmp(category, "common") == 0) {
        if (strcmp(number, "1") == 0) {
            run_result("pthreads_main", pthreads_main(argc, argv));
            return;
        } else if (strcmp(number, "2") == 0) {
            run_result("detached_thread_main", detached_thread_main(argc, argv));
            return;
        } else if (strcmp(number, "3") == 0) {
            run_result("stack_heap_main", stack_heap_main());
            return;
        } else if (strcmp(number, "4") == 0) {
            run_result("volatile_main", volatile_main());
            return;
        }

        printf("common 카테고리의 해당 번호가 없습니다.\n");
        return;
    }

    if (strcmp(category, "sync") == 0) {
        if (strcmp(number, "1") == 0) {
            run_result("sync_race_condition_main", sync_race_condition_main());
            return;
        } else if (strcmp(number, "2") == 0) {
            run_result("sync_cas_main", sync_cas_main());
            return;
        } else if (strcmp(number, "3") == 0) {
            run_result("sync_tas_main", sync_tas_main());
            return;
        } else if (strcmp(number, "4") == 0) {
            run_result("sync_mutex_main", sync_mutex_main());
            return;
        } else if (strcmp(number, "5") == 0) {
            run_result("sync_spinlock_main", sync_spinlock_main());
            return;
        } else if (strcmp(number, "6") == 0) {
            run_result("sync_pthreads_mutex_main", sync_pthreads_mutex_main());
            return;
        } else if (strcmp(number, "7") == 0) {
            run_result("sync_semaphore_main", sync_semaphore_main());
            return;
        } else if (strcmp(number, "7-1") == 0) {
            run_result("sync_atomic_semaphore_main", sync_atomic_semaphore_main());
            return;
        } else if (strcmp(number, "7-2") == 0) {
            run_result("sync_posix_semaphore_main", sync_posix_semaphore_main());
            return;
        } else if (strcmp(number, "8") == 0) {
            run_result("sync_condition_variable_main", sync_condition_variable_main());
            return;
        } else if (strcmp(number, "9-1") == 0) {
            run_result("sync_barrier_spin_old_main", sync_barrier_spin_old_main());
            return;
        } else if (strcmp(number, "9-2") == 0) {
            run_result("sync_barrier_spin_c11_main", sync_barrier_spin_c11_main());
            return;
        } else if (strcmp(number, "9-3") == 0) {
            run_result(
                "sync_barrier_spin_mut_cond_mac_main",
                sync_barrier_spin_mut_cond_mac_main()
            );
            return;
        } else if (strcmp(number, "9-4") == 0) {
            run_result(
                "sync_barrier_spin_posix_bar_linux_main",
                sync_barrier_spin_posix_bar_linux_main()
            );
            return;
        } else if (strcmp(number, "9-5") == 0) {
            run_result("sync_barrier_spin_posix_sem_main", sync_barrier_spin_posix_sem_main());
            return;
        } else if (strcmp(number, "10-1") == 0) {
            run_result("sync_rwlock_spin_old_main", sync_rwlock_spin_old_main());
            return;
        } else if (strcmp(number, "10-2") == 0) {
            run_result("sync_rwlock_spin_pthreads_main", sync_rwlock_spin_pthreads_main());
            return;
        }

        printf("sync 카테고리의 해당 번호가 없습니다.\n");
        return;
    }

    printf("알 수 없는 category 입니다.\n");
}
