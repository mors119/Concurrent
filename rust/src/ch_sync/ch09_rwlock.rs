// read lock 여러 개 가능 write lock 하나만 가능
// read lock이 하나라도 살아 있으면 write lock 대기
use std::sync::{Arc, RwLock};
use std::thread;

pub fn run() {
    // ch04_rwlock::test1()과 다른 점은 Arc로 RwLock을 감싸서 여러 스레드에서 공유할 수 있다는 것이다.
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        //* 최종 안전한 버전
        // write lock 하나만 잡는다.
        // bool을 확인하고 수정하는 작업을 하나의 critical section으로 묶는다.
        let mut flag = val.write().unwrap();

        // gruid 타입이므로 *flag로 실제 값 접근해야 한다.
        if *flag {
            // critical section을 여기에 작성한다.
            *flag = false;
            println!("flag is true");
        }

        /*  1. read guard 유지 중 write 요청 → 데드락 위험
        let flag = val.read().unwrap(); // read lock 획득

        if *flag {
            *val.write().unwrap() = false; // write lock 획득 시도 → 데드락 발생
            println!("flag is true");
        }
        */
        /*  2. bool만 복사 후 read guard 즉시 drop → OK
        let flag = *val.read().unwrap(); // read lock 획득 후 RwLockReadGuard<bool> bool 복사 → read lock 즉시 drop

        if flag {
            *val.write().unwrap() = false; // write lock 획득 시도 → OK
            println!("flag is true");
        }
        */

        /* 3. _flag도 변수라 read guard 유지 → 데드락 위험
        let _flag = val.read().unwrap(); // 클로저 내에서 _flag는 변수이므로 read guard가 살아 있음

        *val.write().unwrap() = false; // write lock 획득 시도 → 데드락 발생
        println!("deadlock");
        */

        /* 4. let _는 즉시 drop → OK, 하지만 읽은 값을 안 쓰므로 의미는 약함
        let _ = val.read().unwrap(); // _는 read lock 획득 후 RwLockReadGuard<bool> 즉시 drop

        *val.write().unwrap() = false; // write lock 획득 시도 → OK
        println!("not deadlock");
        */

        // flag가 scope 끝에서 drop되면서 write lock 해제
    });

    t.join().unwrap();
}
