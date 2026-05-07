use std::sync::{Arc, RwLock}; // Reader Writer Lock
use std::thread;
// 읽기(read)는 여러 명 가능 쓰기(write)는 한 명만 가능
// 읽기가 매우 많고 쓰기가 적은 상황에서 Mutex보다 효율적일 수 있음.

fn test1() {
    // RwLock<u32> 생성
    // - 현재: reader 없음, writer 없음
    let lock = RwLock::new(10);

    {
        // read() = 읽기 lock 획득 (mut 불가)
        // 반환 타입: RwLockReadGuard<i32>
        // 여러 reader는 동시에 존재 가능
        let v1 = lock.read().unwrap();

        // 또 다른 reader 획득 가능
        let v2 = lock.read().unwrap();

        // Dereference(*)가 자동 수행됨
        // 실제로 v1은: *v1
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);

        // → read lock 자동 해제(drop)
    }

    {
        // write() = 쓰기 lock 획득 (mut)
        // 반환 타입: RwLockWriteGuard<i32>
        // writer는 단 하나만 가능
        // writer가 존재하는 동안: 다른 reader 불가 / 다른 writer 불가
        let mut v = lock.write().unwrap();

        // 내부 값 수정
        // v는 guard 타입이므로 *v로 실제 값 접근
        *v = 7;

        println!("v = {}", v);

        // → write lock 자동 해제(drop)
    }
}

fn test2() {
    let data = Arc::new(RwLock::new(100));

    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // 동시에 여러 read 가능
            let value = data.read().unwrap();

            println!("reader {} = {}", i, *value);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn run() {
    test1();
    test2();
}
