use crate::ch_sync::semaphore::Semaphore;
// use std::collections::LinkedList; // LinkedList 보다 VecDeque이 캐시 효율이 더 우수함
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

// Sender<T>
// - 데이터를 channel 안의 공유 큐로 보내는 송신자
// - Clone 가능하므로 여러 송신 스레드가 같은 channel에 데이터를 보낼 수 있음
#[derive(Clone)]
pub struct Sender<T> {
    // 큐의 최대 용량을 제한하는 세마포어
    // send() 전에 wait()를 호출해서 큐가 가득 찼는지 확인
    sem: Arc<Semaphore>,

    // 실제 데이터를 저장하는 공유 큐
    // 여러 스레드가 동시에 접근할 수 있으므로 Mutex로 보호
    // buf: Arc<Mutex<LinkedList<T>>>,
    // - VecDeque가 LinkedList보다 push_back/pop_front가 빠름
    buf: Arc<Mutex<VecDeque<T>>>,

    // receiver가 큐가 비어 있어 wait 중일 때 깨우기 위한 조건 변수
    cond: Arc<Condvar>,
}

impl<T: Send> Sender<T> {
    // 1. 큐에 빈 공간이 생길 때까지 Semaphore에서 대기
    // 2. 큐 lock 획득
    // 3. data를 큐 뒤에 추가
    // 4. receiver 하나에게 데이터가 들어왔다고 알림
    // data: T (송신자가 channel에 넣을 데이터)
    pub fn send(&self, data: T) {
        // 큐가 max 이상이면 여기서 block
        self.sem.wait();

        // 큐에 접근하기 위해 Mutex lock 획득
        let mut buf = self.buf.lock().unwrap();

        // 데이터를 큐 뒤쪽에 삽입
        buf.push_back(data);

        // 큐가 비어서 기다리고 있을 수 있는 receiver를 하나 깨움
        self.cond.notify_one();

        // 함수 종료 시 buf guard가 drop되면서 Mutex unlock
    }
}

// channel 안의 공유 큐에서 데이터를 꺼내는 수신자
pub struct Receiver<T> {
    // recv()가 데이터를 하나 꺼낸 뒤 post()를 호출해서 큐에 빈 공간이 생겼음을 sender에게 알려줌
    sem: Arc<Semaphore>,

    // - 실제 데이터를 저장하는 공유 큐
    // buf: Arc<Mutex<LinkedList<T>>>,
    buf: Arc<Mutex<VecDeque<T>>>,

    // - 큐가 비어 있을 때 receiver가 기다리는 조건 변수
    cond: Arc<Condvar>,
}

impl<T> Receiver<T> {
    // recv() 큐에서 데이터 하나를 꺼내 반환,
    // 큐가 비어 있으면 데이터가 들어올 때까지 block
    pub fn recv(&self) -> T {
        // 큐 lock 획득
        let mut buf = self.buf.lock().unwrap();

        loop {
            // 큐 앞쪽에서 데이터 꺼내기 시도
            if let Some(data) = buf.pop_front() {
                // 데이터를 하나 꺼냈으므로 큐에 빈 공간이 생김
                // 대기 중인 sender 하나를 깨울 수 있음
                self.sem.post();

                return data;
            }

            // 큐가 비어 있으면 대기

            // wait()는:
            // 1. 현재 Mutex lock을 잠시 풀고
            // 2. receiver 스레드를 재우고
            // 3. sender가 notify_one()을 호출하면 깨우고
            // 4. 다시 Mutex lock을 획득한 뒤
            // 5. 새 MutexGuard를 반환
            buf = self.cond.wait(buf).unwrap();
        }
    }
}

// 최대 max개까지 버퍼링 가능한 bounded channel 생성
// 반환: Sender<T>: 송신자 Receiver<T>: 수신자
pub fn channel<T>(max: isize) -> (Sender<T>, Receiver<T>) {
    assert!(max > 0);

    // 큐의 최대 용량 제한용 세마포
    let sem = Arc::new(Semaphore::new(max));

    // 실제 공유 큐
    let buf = Arc::new(Mutex::new(VecDeque::new()));

    // receiver 대기/깨우기용 조건 변수
    let cond = Arc::new(Condvar::new());

    let tx = Sender {
        sem: Arc::clone(&sem),
        buf: Arc::clone(&buf),
        cond: Arc::clone(&cond),
    };

    let rx = Receiver { sem, buf, cond };

    (tx, rx)
}

pub fn run() {
    // 최대 버퍼 크기 4인 bounded channel 생성
    let (tx, rx) = channel(4);

    // 생성한 thread handle 저장용 벡터
    let mut handles = Vec::new();

    // recv()를 반복 호출하면서 sender들이 보낸 데이터를 출력
    let receiver_thread = thread::spawn(move || {
        // 총 몇 개를 받을지 카운트
        // sender 3개 각 sender당 5개 전송
        // 총 15개 수신 예정
        for _ in 0..15 {
            // recv() 큐가 비어 있으면 자동 대기
            // 데이터 들어오면 깨어남
            let data = rx.recv();

            println!("recv => {:?}", data);
        }

        println!("receiver finished");
    });

    handles.push(receiver_thread);

    // sender 3개 생성
    for thread_id in 0..3 {
        // Sender clone은 shared queue 공유
        let tx_clone = tx.clone();

        let sender_thread = thread::spawn(move || {
            // 각 sender가 5개씩 메시지 전송
            for message_index in 0..5 {
                // (sender_id, message_index)
                let data = (thread_id, message_index);

                println!("send => {:?}", data);

                // bounded channel send
                // 큐가 가득 차면 여기서 block
                tx_clone.send(data);
            }

            println!("sender {} finished", thread_id);
        });

        handles.push(sender_thread);
    }

    // 모든 thread 종료 대기
    for handle in handles {
        handle.join().unwrap();
    }

    println!("all threads finished");
}
