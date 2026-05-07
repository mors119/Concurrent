use std::hint::spin_loop;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

// Lamport Bakery Algorithm
// - 빵집 번호표처럼 각 스레드가 ticket을 뽑는다.
// - ticket 번호가 작은 스레드가 먼저 임계 구역에 들어간다.
// - ticket이 같으면 thread id가 작은 쪽이 먼저 들어간다.
// - 조금 더 rust스러운 건 mutex를 사용해서 구현하는 것이다.

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100000;

// BakeryLock
// - Mutex 없이 atomic 변수만으로 mutual exclusion을 구현하는 학습용 lock
struct BakeryLock {
    // entering[i] == true
    // - i번 스레드가 ticket을 고르는 중이라는 뜻
    entering: [AtomicBool; NUM_THREADS],

    // tickets[i] == 0
    // - i번 스레드는 lock 경쟁에 참여하지 않음
    //
    // tickets[i] > 0
    // - i번 스레드가 해당 ticket 번호를 들고 기다리는 중
    tickets: [AtomicU64; NUM_THREADS],
}

impl BakeryLock {
    fn new() -> Self {
        Self {
            // AtomicBool은 Copy 타입이 아니므로 배열 초기화에 from_fn 사용
            entering: std::array::from_fn(|_| AtomicBool::new(false)),

            // 0은 None 역할
            tickets: std::array::from_fn(|_| AtomicU64::new(0)),
        }
    }

    // lock()
    // - idx: 현재 스레드 번호
    // - 반환된 LockGuard가 살아 있는 동안 lock 획득 상태
    fn lock(&self, idx: usize) -> LockGuard<'_> {
        // 1. ticket을 고르는 중이라고 표시
        self.entering[idx].store(true, Ordering::SeqCst);

        // 2. 현재 ticket 중 최댓값 찾기
        let mut max_ticket = 0;

        for i in 0..NUM_THREADS {
            let ticket = self.tickets[i].load(Ordering::SeqCst);
            max_ticket = max_ticket.max(ticket);
        }

        // 3. 내 ticket = 현재 최댓값 + 1
        let my_ticket = max_ticket + 1;
        self.tickets[idx].store(my_ticket, Ordering::SeqCst);

        // 4. ticket 선택 완료
        self.entering[idx].store(false, Ordering::SeqCst);

        // 5. 다른 모든 스레드와 우선순위 비교
        for i in 0..NUM_THREADS {
            if i == idx {
                continue;
            }

            // 상대 스레드가 ticket을 고르는 중이면 기다림
            while self.entering[i].load(Ordering::SeqCst) {
                spin_loop();
            }

            loop {
                let other_ticket = self.tickets[i].load(Ordering::SeqCst);

                // other_ticket == 0
                // - 상대 스레드는 lock 경쟁에 참여하지 않음
                if other_ticket == 0 {
                    break;
                }

                // 우선순위 비교:
                // (ticket 번호, thread id)가 작은 쪽이 먼저 들어감
                let i_go_first = other_ticket < my_ticket || (other_ticket == my_ticket && i < idx);

                // 상대가 먼저라면 기다림
                if i_go_first {
                    spin_loop();
                    continue;
                }

                // 내가 먼저라면 통과
                break;
            }
        }

        LockGuard { lock: self, idx }
    }

    // unlock()
    // - ticket을 0으로 되돌려서 lock 경쟁에서 빠짐
    fn unlock(&self, idx: usize) {
        self.tickets[idx].store(0, Ordering::SeqCst);
    }
}

// LockGuard
// - Rust의 MutexGuard처럼 scope 기반으로 lock을 관리
// - 이 값이 drop될 때 자동으로 unlock 호출
struct LockGuard<'a> {
    lock: &'a BakeryLock,
    idx: usize,
}

impl Drop for LockGuard<'_> {
    fn drop(&mut self) {
        self.lock.unlock(self.idx);
    }
}

pub fn run() {
    let lock = Arc::new(BakeryLock::new());
    let count = Arc::new(AtomicU64::new(0));

    let mut handles = Vec::new();

    for thread_id in 0..NUM_THREADS {
        let lock = Arc::clone(&lock);
        let count = Arc::clone(&count);

        let handle = thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                // lock 획득
                let _guard = lock.lock(thread_id);

                // 임계 구역
                //
                // BakeryLock이 제대로 동작한다면
                // 이 구간에는 한 번에 하나의 스레드만 들어온다.
                let current = count.load(Ordering::SeqCst);
                count.store(current + 1, Ordering::SeqCst);

                // _guard가 여기서 drop되면서 unlock
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = count.load(Ordering::SeqCst);

    println!("COUNT = {} (expected = {})", result, NUM_LOOP * NUM_THREADS);
}
