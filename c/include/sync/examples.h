#ifndef SYNC_EXAMPLES_H
#define SYNC_EXAMPLES_H

int sync_race_condition_main(void);
int sync_cas_main(void);
int sync_tas_main(void);
int sync_mutex_main(void);
int sync_spinlock_main(void);
int sync_pthreads_mutex_main(void);
int sync_semaphore_main(void);
int sync_atomic_semaphore_main(void);
int sync_posix_semaphore_main(void);
int sync_condition_variable_main(void);
int sync_barrier_spin_old_main(void);
int sync_barrier_spin_c11_main(void);
int sync_barrier_spin_mut_cond_mac_main(void);
int sync_barrier_spin_posix_bar_linux_main(void);
int sync_barrier_spin_posix_sem_main(void);
int sync_rwlock_spin_old_main(void);
int sync_rwlock_spin_pthreads_main(void);

#endif
