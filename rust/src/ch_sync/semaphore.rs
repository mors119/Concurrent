use std::sync::{Condvar, Mutex};

// Semaphore 동시에 허용되는 작업 개수를 제한하는 동기화 도구
// - cnt = 현재 큐에 들어가 있는 데이터 개수
// - max = 큐 최대 용량
// send()는 wait()를 호출해서 cnt를 증가시키고,
// recv()는 post()를 호출해서 cnt를 감소시킨다.
pub struct Semaphore {
    // - 현재 사용 중인 slot 개수 cnt를 보호
    mutex: Mutex<isize>,
    cond: Condvar,
    max: isize,
}

impl Semaphore {
    // 동시에 허용할 최대 개수
    pub fn new(max: isize) -> Self {
        Semaphore {
            mutex: Mutex::new(0),
            cond: Condvar::new(),
            max,
        }
    }

    // wait() 빈 slot이 생길 때까지 기다린 뒤 cnt를 증가시킴
    // send() 전에 호출됨
    pub fn wait(&self) {
        let mut cnt = self.mutex.lock().unwrap();

        // cnt가 max 이상이면 큐가 가득 찬 상태
        while *cnt >= self.max {
            cnt = self.cond.wait(cnt).unwrap();
        }

        // 빈 slot 하나 사용
        *cnt += 1;
    }

    // post() 사용 중이던 slot 하나를 반납
    // recv()가 데이터를 꺼낸 뒤 호출됨
    pub fn post(&self) {
        let mut cnt = self.mutex.lock().unwrap();

        // slot 하나 반납
        *cnt -= 1;

        // 기다리는 sender 하나를 깨움
        self.cond.notify_one();
    }
}
