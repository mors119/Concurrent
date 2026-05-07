use std::sync::{Arc, Condvar, Mutex}; // ❶ CondVar(Condition Variable) 조건 변수:
use std::thread;

// Condvar: 어떤 조건이 true가 될 때까지 스레드를 잠재우고, 조건이 바뀌면 깨운다.

// Condvar 타입의 변수가 조건 변수이며
// Mutex와 Condvar를 포함하는 튜플이 Arc에 포함되어 전달된다
// 여러 스레드가 공유하는 "상태값 + 조건 변수" 묶음
fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
    // Arc 안의 튜플에서 Mutex와 Condvar를 참조로 꺼낸다.
    let &(ref lock, ref cvar) = &*p; // == let (lock, cvar) = &*p;

    // 먼저, 뮤텍스 락 획득
    let mut started = lock.lock().unwrap(); // 여기서 started는 bool이 아닌 MutexGuard<bool>다.
    while !*started {
        // Mutex 안의 공유 변수가 false인 동안 루프(깨웠는데 조건을 만족하지 않을 수 있으므로 루프 사용)

        started = cvar.wait(started).unwrap(); // Condvar의 핵심 (wait로 대기)
        // 1. 현재 잡고 있던 Mutex lock을 잠시 풀어준다.
        // 2. 현재 스레드를 잠든 상태로 만든다.
        // 3. 다른 스레드가 notify하면 깨어난다.
        // 4. 깨어난 뒤 다시 Mutex lock을 획득한다.
        // 5. 새 MutexGuard<bool>를 반환한다.

        // lock을 풀고 잠들어야하기 때문에 MutexGuard<bool>를 받아야함.
        // 아니면 데드락 발생
    }

    // 다음과 같이 wait_while을 사용할 수도 있음
    // cvar.wait_while(started, |started| !*started).unwrap();

    println!("child {}", id);
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    // 먼저 뮤텍스 락 획득
    let mut started = lock.lock().unwrap();
    *started = true; // 공유 상태 started = true로 변경
    cvar.notify_all(); // 알림 (조건 변수를 기다리는 모든 스레드를 깨운다)
    println!("parent");
}

pub fn run() {
    // 뮤텍스와 조건 변수를 작성
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone(); // == let pair1 = Arc::clone(&pair0);
    let pair2 = pair0.clone(); // == let pair2 = Arc::clone(&pair0);

    let c0 = thread::spawn(move || child(0, pair0));
    let c1 = thread::spawn(move || child(1, pair1));
    let p = thread::spawn(move || parent(pair2));

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}
