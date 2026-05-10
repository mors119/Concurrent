use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;

/// Deadlock 4 Conditions
///
/// 1. Mutual Exclusion
///    - 자원을 한 번에 하나만 사용할 수 있음
///    - Mutex, RwLock write lock 등이 여기에 해당
///
/// 2. Hold and Wait
///    - 어떤 자원을 이미 들고 있는 상태에서 다른 자원을 기다림
///
/// 3. No Preemption
///    - 다른 스레드가 가진 자원을 강제로 빼앗을 수 없음
///    - MutexGuard는 소유자가 drop해야 풀림
///
/// 4. Circular Wait
///    - A는 B가 가진 자원을 기다리고,
///      B는 A가 가진 자원을 기다리는 순환 구조
///
/// 데드락은 보통 이 4개가 동시에 만족될 때 발생한다.

/// 1. lock 순서 반대로 잡는 데드락

pub fn deadlock_lock_order() {
    let c0 = Arc::new(Mutex::new(()));
    let c1 = Arc::new(Mutex::new(()));

    let c0_p0 = Arc::clone(&c0);
    let c1_p0 = Arc::clone(&c1);

    let p0 = thread::spawn(move || {
        // [1. Mutual Exclusion]
        // c0는 Mutex이므로 한 번에 하나의 스레드만 획득 가능
        let _a = c0_p0.lock().unwrap();

        // [2. Hold and Wait]
        // p0는 c0를 들고 있는 상태에서 c1을 기다림
        let _b = c1_p0.lock().unwrap();

        println!("p0 eating");
    });

    let p1 = thread::spawn(move || {
        // [1. Mutual Exclusion]
        // c1도 Mutex이므로 한 번에 하나의 스레드만 획득 가능
        let _a = c1.lock().unwrap();

        // [2. Hold and Wait]
        // p1는 c1을 들고 있는 상태에서 c0를 기다림

        // [3. No Preemption]
        // p0가 가진 c0를 p1이 강제로 빼앗을 수 없음

        // [4. Circular Wait]
        // p0: c0 보유 → c1 대기
        // p1: c1 보유 → c0 대기

        // 서로 상대가 가진 자원을 기다리므로 데드락 가능
        let _b = c0.lock().unwrap();

        // 해결 방법:
        // 모든 스레드가 lock 획득 순서를 통일한다.
        // 예: 항상 c0 -> c1 순서

        // let _a = c0.lock().unwrap();
        // let _b = c1.lock().unwrap();

        // 이렇게 하면 [4. Circular Wait] 조건을 제거하므로 데드락 방지 가능

        println!("p1 eating");
    });

    p0.join().unwrap();
    p1.join().unwrap();
}

pub fn fixed_lock_order() {
    let c0 = Arc::new(Mutex::new(()));
    let c1 = Arc::new(Mutex::new(()));

    let c0_p0 = Arc::clone(&c0);
    let c1_p0 = Arc::clone(&c1);

    let p0 = thread::spawn(move || {
        // 항상 c0 -> c1 순서
        let _a = c0_p0.lock().unwrap();
        let _b = c1_p0.lock().unwrap();

        println!("p0 eating");
    });

    let p1 = thread::spawn(move || {
        // p1도 c0 -> c1 순서
        // lock 순서를 통일하면 순환 대기 조건이 깨진다.
        let _a = c0.lock().unwrap();
        let _b = c1.lock().unwrap();

        println!("p1 eating");
    });

    p0.join().unwrap();
    p1.join().unwrap();
}

/// 2. RwLock read lock을 들고 write lock을 요청하는 데드락

pub fn deadlock_rwlock_upgrade() {
    let flag = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        // [1. Mutual Exclusion]
        // RwLock의 write lock은 단 하나만 가능
        // read lock이 살아 있으면 write lock은 대기해야 함
        let read_guard = flag.read().unwrap();

        if *read_guard {
            // [2. Hold and Wait]
            // 현재 read_guard를 들고 있는 상태에서 write lock을 기다림

            // [3. No Preemption]
            // write lock이 read_guard를 강제로 해제할 수 없음

            // [4. Circular Wait 비슷한 자기 대기]
            // 같은 스레드가 read lock을 들고 write lock을 기다림
            // write lock은 read lock이 풀리기를 기다림

            // 결과적으로 자기 자신이 가진 read lock 때문에 멈출 수 있음
            *flag.write().unwrap() = false;
        }
    });

    t.join().unwrap();
}

pub fn fixed_rwlock_copy_value() {
    let flag = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        // bool은 Copy 타입
        // read lock으로 값을 복사한 뒤, 이 문장 끝에서 read guard가 바로 drop됨
        let is_true = *flag.read().unwrap();

        if is_true {
            // read guard가 이미 drop되었으므로 write lock 획득 가능
            *flag.write().unwrap() = false;
            println!("flag changed");
        }
    });

    t.join().unwrap();
}

pub fn fixed_rwlock_write_once() {
    let flag = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        // 검사와 수정을 하나의 write lock 안에서 처리
        // read -> write 업그레이드 문제가 사라짐
        let mut guard = flag.write().unwrap();

        if *guard {
            *guard = false;
            println!("flag changed");
        }
    });

    t.join().unwrap();
}

/// 3. join을 lock 잡은 상태에서 호출하는 데드락

pub fn deadlock_join_while_locked() {
    let data = Arc::new(Mutex::new(0));
    let data_child = Arc::clone(&data);

    // [1. Mutual Exclusion]
    // data는 Mutex로 보호됨
    let mut guard = data.lock().unwrap();

    let child = thread::spawn(move || {
        // child는 data lock을 얻어야 종료 가능
        let mut n = data_child.lock().unwrap();
        *n += 1;
    });

    // [2. Hold and Wait]
    // main thread는 data lock을 들고 child 종료를 기다림

    // [3. No Preemption]
    // child는 main이 가진 data lock을 강제로 빼앗을 수 없음

    // [4. Circular Wait]
    // main: data lock 보유 → child 종료 대기
    // child: 종료하려면 data lock 필요 → main이 가진 lock 대기

    // 서로 기다리므로 데드락 가능
    child.join().unwrap();

    *guard += 1;
}

pub fn fixed_join_after_drop() {
    let data = Arc::new(Mutex::new(0));
    let data_child = Arc::clone(&data);

    {
        let mut guard = data.lock().unwrap();
        *guard += 1;

        // 이 블록 끝에서 guard drop
        // 즉 Mutex unlock
    }

    let child = thread::spawn(move || {
        let mut n = data_child.lock().unwrap();
        *n += 1;
    });

    // lock을 들고 있지 않은 상태에서 join
    child.join().unwrap();

    println!("data = {}", *data.lock().unwrap());
}

/// 4. callback을 lock 안에서 호출하는 데드락

pub fn deadlock_callback_inside_lock() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_for_callback = Arc::clone(&data);

    let callback = || {
        // callback도 같은 Mutex를 다시 잡으려고 함
        let mut v = data_for_callback.lock().unwrap();
        v.push(4);
    };

    // [1. Mutual Exclusion]
    // data는 Mutex로 보호됨
    let _guard = data.lock().unwrap();

    // [2. Hold and Wait]
    // data lock을 들고 있는 상태에서 callback 호출

    // [3. No Preemption]
    // callback은 현재 스레드가 들고 있는 lock을 강제로 빼앗을 수 없음

    // [4. Circular Wait 비슷한 자기 대기]
    // 현재 코드가 data lock을 들고 있음
    // callback도 data lock을 다시 요청함
    // std::sync::Mutex는 reentrant mutex가 아니므로 데드락 가능
    callback();
}

pub fn fixed_callback_outside_lock() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_for_callback = Arc::clone(&data);

    let callback = || {
        let mut v = data_for_callback.lock().unwrap();
        v.push(4);
    };

    {
        let guard = data.lock().unwrap();

        // lock 안에서는 필요한 작업만 짧게 수행
        println!("len = {}", guard.len());

        // 여기서 guard drop
    }

    // lock 밖에서 callback 호출
    callback();

    println!("{:?}", *data.lock().unwrap());
}

/// 5. Condvar 없이 Mutex만으로 대기하다가 꼬이는 상황

pub fn bad_busy_wait_with_mutex() {
    let ready = Arc::new(Mutex::new(false));
    let ready_worker = Arc::clone(&ready);

    let worker = thread::spawn(move || {
        loop {
            let guard = ready_worker.lock().unwrap();

            if *guard {
                break;
            }

            // 이 예제는 반드시 데드락이라고 보긴 어렵지만 나쁜 대기 방식이다.

            // 문제:
            // - 반복적으로 lock을 잡았다 풀었다 함
            // - CPU를 낭비함
            // - lock scope를 잘못 잡으면 setter가 lock을 못 얻어 멈출 수 있음
            drop(guard);
        }

        println!("worker starts");
    });

    let setter = thread::spawn(move || {
        let mut guard = ready.lock().unwrap();
        *guard = true;
    });

    worker.join().unwrap();
    setter.join().unwrap();
}

pub fn fixed_condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let worker_pair = Arc::clone(&pair);

    let worker = thread::spawn(move || {
        let (lock, cvar) = &*worker_pair;

        let mut ready = lock.lock().unwrap();

        while !*ready {
            // Condvar::wait는:
            // 1. Mutex lock을 잠시 해제
            // 2. 현재 스레드를 sleep
            // 3. notify를 받으면 깨어남
            // 4. 다시 Mutex lock을 획득

            // 따라서 lock을 들고 계속 대기하는 문제를 피할 수 있음
            ready = cvar.wait(ready).unwrap();
        }

        println!("worker starts");
    });

    let setter = thread::spawn(move || {
        let (lock, cvar) = &*pair;

        let mut ready = lock.lock().unwrap();
        *ready = true;

        // 대기 중인 worker를 깨움
        cvar.notify_one();
    });

    worker.join().unwrap();
    setter.join().unwrap();
}

/// 6. try_lock으로 데드락 회피

pub fn avoid_deadlock_with_try_lock() {
    let a = Arc::new(Mutex::new(()));
    let b = Arc::new(Mutex::new(()));

    let a0 = Arc::clone(&a);
    let b0 = Arc::clone(&b);

    let t0 = thread::spawn(move || {
        loop {
            let lock_a = a0.lock().unwrap();

            // 두 번째 lock을 못 잡으면 첫 번째 lock을 내려놓고 재시도
            // [2. Hold and Wait] 조건을 약하게 만든다.
            if let Ok(lock_b) = b0.try_lock() {
                println!("t0 acquired both");

                drop(lock_b);
                drop(lock_a);
                break;
            }

            drop(lock_a);
            thread::yield_now();
        }
    });

    let t1 = thread::spawn(move || {
        loop {
            let lock_b = b.lock().unwrap();

            if let Ok(lock_a) = a.try_lock() {
                println!("t1 acquired both");

                drop(lock_a);
                drop(lock_b);
                break;
            }

            drop(lock_b);
            thread::yield_now();
        }
    });

    t0.join().unwrap();
    t1.join().unwrap();
}

/// 7. 여러 자원을 하나의 Mutex로 묶어서 해결

struct Forks {
    c0: (),
    c1: (),
}

pub fn fixed_single_mutex() {
    let forks = Arc::new(Mutex::new(Forks { c0: (), c1: () }));

    let forks0 = Arc::clone(&forks);

    let p0 = thread::spawn(move || {
        // c0, c1을 따로 lock하지 않고 하나의 Mutex로 보호

        // 장점:
        // - 여러 lock 사이의 순환 대기 자체가 사라짐

        // 단점:
        // - 병렬성이 줄어듦
        let _guard = forks0.lock().unwrap();

        println!("p0 eating");
    });

    let p1 = thread::spawn(move || {
        let _guard = forks.lock().unwrap();

        println!("p1 eating");
    });

    p0.join().unwrap();
    p1.join().unwrap();
}

pub fn run() {
    // 데드락 예제는 실제 호출하면 멈출 수 있음.
    // deadlock_lock_order();
    // deadlock_rwlock_upgrade();
    // deadlock_join_while_locked();
    // deadlock_callback_inside_lock();

    fixed_lock_order();
    fixed_rwlock_copy_value();
    fixed_rwlock_write_once();
    fixed_join_after_drop();
    fixed_callback_outside_lock();
    fixed_condvar();
    avoid_deadlock_with_try_lock();
    fixed_single_mutex();
}
