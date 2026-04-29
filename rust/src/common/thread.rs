use std::thread;
use std::time::Duration;

// pthread_create + pthread_join 대응
fn pthreads_like_main() {
    const NUM_THREADS: usize = 10;

    let mut handles = Vec::new();

    for id in 0..NUM_THREADS {
        // thread::spawn = 새 스레드 생성

        // move:
        // - 현재 스코프의 id 값을 스레드 안으로 소유권 이동
        // - id는 usize이고 Copy 타입이라 실제로는 값 복사처럼 동작
        let handle = thread::spawn(move || {
            for i in 0..5 {
                println!("id = {}, i = {}", id, i);
                thread::sleep(Duration::from_secs(1));
            }

            // pthread의 return "finished!"와 유사
            "finished!"
        });

        handles.push(handle);
    }

    for handle in handles {
        // join = 스레드 종료 대기

        // unwrap:
        // - 스레드 내부 panic이 없었다고 가정
        // - 실무에서는 match로 처리 가능
        let msg = handle.join().unwrap();
        println!("msg = {}", msg);
    }
}

// detached thread 대응
fn detached_thread_like_main() {
    // JoinHandle을 변수에 저장하지 않거나 drop하면 Rust에서는 사실상 detached처럼 동작.
    // 단, 프로세스가 종료되면 해당 스레드도 같이 종료될 수 있으므로 main 스레드가 충분히 살아 있어야 합니다.
    thread::spawn(|| {
        for i in 0..5 {
            println!("i = {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    // C의 sleep(7)과 같은 역할
    // main이 너무 빨리 끝나면 spawned thread도 끝까지 실행되지 못할 수 있음
    thread::sleep(Duration::from_secs(7));
}

// stack / heap 공유
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct ThreadArg {
    heap_ptr: Arc<Mutex<i32>>,
    thread_id: i32,
}

fn stack_heap_like_main() {
    // Arc:
    // - Atomic Reference Counted
    // - 여러 스레드가 같은 데이터를 소유할 수 있게 해주는 스마트 포인터

    // Mutex:
    // - Mutual Exclusion
    // - 동시에 하나의 스레드만 내부 값을 수정하게 보호
    let heap_val = Arc::new(Mutex::new(0));

    let arg1 = ThreadArg {
        heap_ptr: Arc::clone(&heap_val),
        thread_id: 1,
    };

    let arg2 = ThreadArg {
        heap_ptr: Arc::clone(&heap_val),
        thread_id: 2,
    };

    let t1 = thread::spawn(move || {
        thread_stack_func(arg1);
    });

    let t2 = thread::spawn(move || {
        thread_stack_func(arg2);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let final_value = heap_val.lock().unwrap();
    println!("\n[Main] final heap value = {}", *final_value);
}

fn thread_stack_func(arg: ThreadArg) {
    // 스택 변수
    // 각 스레드마다 따로 존재
    let mut stack_val = 0;

    for _ in 0..3 {
        stack_val += 1;

        // lock()으로 공유 heap 값을 잠금
        // 이 블록 안에서는 한 스레드만 heap 값을 수정 가능
        {
            let mut heap = arg.heap_ptr.lock().unwrap();
            *heap += 1;

            println!(
                "[Thread {}] stack={}, heap={}",
                arg.thread_id, stack_val, *heap
            );
        }

        thread::sleep(Duration::from_secs(1));
    }
}

pub fn run() {
    println!("--- pthread join style ---");
    pthreads_like_main();

    println!("--- detached style ---");
    detached_thread_like_main();

    println!("--- stack / heap style ---");
    stack_heap_like_main();
}
