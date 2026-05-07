use std::sync::{Arc, Condvar, Mutex};
use std::thread;

// 공유 상태 타입을 별칭으로 분리
// Arc<(Mutex<bool>, Condvar)>를 매번 쓰면 읽기 어렵기 때문에 별칭 사용
type StartSignal = Arc<(Mutex<bool>, Condvar)>;

fn child(id: u64, signal: StartSignal) {
    // signal: Arc<(Mutex<bool>, Condvar)>
    // - 여러 스레드가 같은 시작 신호를 공유하기 위한 스마트 포인터
    // - 내부에는 Mutex<bool>과 Condvar가 들어 있음

    let (lock, cvar) = &*signal;

    // lock: &Mutex<bool>
    // - started 상태값을 보호하는 Mutex
    //
    // cvar: &Condvar
    // - started 값이 true가 될 때까지 child 스레드를 잠재우는 조건 변수

    let started = lock.lock().unwrap();

    // started: MutexGuard<bool>
    // - Mutex lock을 획득한 상태에서 bool 값을 접근할 수 있는 guard
    //
    // wait_while은 조건이 true인 동안 현재 스레드를 대기시킨다.
    // 여기서는 !*started, 즉 started == false인 동안 기다린다.
    let _started = cvar
        .wait_while(started, |started| {
            // started: &mut bool
            // - Mutex 안에 들어 있는 bool 값에 대한 mutable reference
            // - true가 되면 대기를 멈추고 child가 진행한다.
            !*started
        })
        .unwrap();

    println!("child {}", id);
}

fn parent(signal: StartSignal) {
    // signal: Arc<(Mutex<bool>, Condvar)>
    // - child들과 공유하는 시작 신호

    let (lock, cvar) = &*signal;

    // lock을 잡고 started 값을 true로 변경한다.
    let mut started = lock.lock().unwrap();

    // started: MutexGuard<bool>
    // - 내부 bool 값을 수정하기 위해 mut 필요
    *started = true;

    // notify_all:
    // - 이 Condvar에서 기다리고 있는 모든 스레드를 깨운다.
    // - 단, 실제로 깨어난 스레드는 다시 Mutex lock을 얻어야 진행 가능하다.
    cvar.notify_all();

    println!("parent");
}

pub fn run() {
    let signal: StartSignal = Arc::new((Mutex::new(false), Condvar::new()));

    let child_signal_0 = Arc::clone(&signal);
    let child_signal_1 = Arc::clone(&signal);
    let parent_signal = Arc::clone(&signal);

    let c0 = thread::spawn(move || {
        child(0, child_signal_0);
    });

    let c1 = thread::spawn(move || {
        child(1, child_signal_1);
    });

    let p = thread::spawn(move || {
        parent(parent_signal);
    });

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}
