use std::sync::{Arc, Barrier};
use std::thread;

// Barrier: 여러 스레드가 특정 지점까지 모두 도착할 때까지 기다리는 동기화 도구
// 10개의 worker thread가 모두 준비 완료될 때까지 대기 → 전부 도착하면 동시에 진행

fn test1() {
    // thread::spawn의 반환값인 JoinHandle 저장 벡터
    // 나중에 join() 호출해서 모든 스레드 종료 대기 가능
    let mut handles = Vec::new();

    // Barrier 생성
    // Barrier::new(10) = 총 10개의 스레드가 wait()에 도착해야 barrier가 열림
    // Arc 사용 이유: 여러 스레드가 같은 Barrier 공유해야 하기 때문
    let barrier = Arc::new(Barrier::new(10));

    // 10개 스레드 생성
    for i in 0..10 {
        // Arc clone: ref_count 증가만 수행
        // 모든 스레드가 같은 Barrier 공유
        let b = Arc::clone(&barrier);
        let th = thread::spawn(move || {
            println!("thread {} before wait", i);

            // barrier wait
            // 현재 스레드는 - 다른 모든 스레드가 wait()에 도착할 때까지 대기
            // 10개 모두 도착하면 → 전부 동시에 통과
            b.wait();

            println!("thread {} after wait", i);
        });

        handles.push(th);
    }

    // 모든 스레드 종료 대기
    for th in handles {
        th.join().unwrap();
    }
}

pub fn run() {
    test1();
}
