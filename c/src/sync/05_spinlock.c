#include <stdatomic.h>
#include <stdbool.h>
#include <pthread.h>
#include <stdio.h>

#include "sync/examples.h"

// 보다 현대적인 TTAS spin lock 예시
typedef struct {
    atomic_bool locked;
} spinlock_t;

static void spinlock_init(spinlock_t *lock) {
    atomic_init(&lock->locked, false);
}

static void spinlock_acquire(spinlock_t *lock) {
    for (;;) {
        while (atomic_load_explicit(&lock->locked, memory_order_relaxed)) {
            // busy wait
        }

        if (!atomic_exchange_explicit(
                &lock->locked,
                true,
                memory_order_acquire
            )) {
            break;
        }
    }
}

static void spinlock_release(spinlock_t *lock) {
    atomic_store_explicit(
        &lock->locked,
        false,
        memory_order_release
    );
}

static pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
static int counter = 0;

static void some_func_2(void) {
    pthread_mutex_lock(&mutex);
    counter++;
    pthread_mutex_unlock(&mutex);
}

int sync_spinlock_main(void) {
    spinlock_t lock;

    spinlock_init(&lock);
    spinlock_acquire(&lock);
    printf("spinlock critical section\n");
    spinlock_release(&lock);

    some_func_2();
    printf("pthread mutex counter = %d\n", counter);

    return 0;
}
