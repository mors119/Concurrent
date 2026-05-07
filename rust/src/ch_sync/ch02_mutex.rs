use std::sync::{Arc, Mutex}; // ❶
use std::thread;

// Arc(Atomic + Reference Counted):
// Reference Counted → 여러 곳에서 같은 데이터를 공유
// (Rc는 단일 스레드 전용)
// Atomic → 여러 스레드에서도 안전하게 공유
// 러스트는 소유권 이동이 일어나기 때문에 ownership이 여러개 필요하고 Arc가 이걸 제공

// Arc → 여러 스레드가 같은 데이터를 공유 가능 (여러 owner 허용)
// Mutex → 동시에 한 스레드만 접근 가능 (동시에 한 명만 읽기/쓰기 가능)
// mutex는 lock을 획득해야만 접근할 수 있기 때문에 .lock()이 필요

/*
  실제로는 다음과 같은 형태가 됨.

  Thread 1 ─┐
            ├── Arc ── Mutex ── u64 값
  Thread 2 ─┘

  그리고 Arc가 0되면 메모리 자동 해제됨.
*/

fn test1() {
    // 모두 같은 "hello"를 봄. ref_count = 3
    let msg = Arc::new(String::from("hello"));
    let a = Arc::clone(&msg);
    let b = Arc::clone(&msg);

    println!("{}", msg);
    println!("{}", a);
    println!("{}", b);
}

fn some_func(lock: Arc<Mutex<u64>>) {
    loop {
        // 록을 하지 않으면 Mutex 타입 안의 값은 참조 불가
        // 블록이 끝나면 자동 unlock
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{}", *val);
    }
}

fn test2() {
    // Arc::new - reference counting 시작 / Mutex::new(0) 초기값 0, lock 보호 시작
    let lock0 = Arc::new(Mutex::new(0));

    // 참조 카운터가 인크리먼트될 뿐이며 내용은 클론되지 않음
    // Arc clone은 깊은 복사가 아닌 공유 포인터 하나 추가, 실제로는 참조 카운트 += 1만 수행
    let lock1 = lock0.clone();

    // 스레드 생성
    let th0 = thread::spawn(move || {
        // 클로저 내부로 ownership 이동
        // (클로저 내 변수로 Arc ownership 이동, 하지만 Arc이므로 실제 데이터는 공유됨)
        some_func(lock0);
    });

    let th1 = thread::spawn(move || {
        some_func(lock1);
    });

    // 약속
    th0.join().unwrap();
    th1.join().unwrap();
}

fn test3() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        // 10개 스레드
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // 각 스레드가 1000 증가
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result = {}", *counter.lock().unwrap()); // 결과 10000
}

pub fn run() {
    test1();
    test2();
    test3();
}
