use crate::ch_sync::semaphore::Semaphore;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 8;
const SEM_NUM: isize = 4;

// AtomicUsize는 내부적으로 안전한 원자적 변경을 제공하므로
// static mut가 아니라 static으로 선언하면 된다. (굳이 unsafe 사용할 필요 없음.)
static CNT: AtomicUsize = AtomicUsize::new(0);

pub fn run() {
    let mut v = Vec::new();

    // SEM_NUM만큼 동시 실행 가능한 세마포어
    let sem = Arc::new(Semaphore::new(SEM_NUM));

    for i in 0..NUM_THREADS {
        // 같은 Semaphore를 여러 스레드가 공유
        let s = Arc::clone(&sem);

        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                // 세마포어 slot 하나 획득
                // 현재 동시에 실행 중인 스레드가 SEM_NUM개면 여기서 대기
                s.wait();

                // 현재 critical section에 들어온 스레드 수 +1
                CNT.fetch_add(1, Ordering::SeqCst);

                // 현재 critical section 안의 스레드 수 읽기
                let n = CNT.load(Ordering::SeqCst);

                println!("semaphore: i = {}, CNT = {}", i, n);

                // 세마포어가 제대로 동작한다면 CNT는 항상 SEM_NUM 이하여야 함
                assert!((n as isize) <= SEM_NUM);

                // critical section에서 나가기 전 카운트 -1
                CNT.fetch_sub(1, Ordering::SeqCst);

                // 세마포어 slot 반납
                s.post();
            }
        });

        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}
