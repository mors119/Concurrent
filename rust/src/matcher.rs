use crate::ch_sync;

pub fn run(args: &[String]) {
    match args[1].as_str() {
        "sync" => match args[2].as_str() {
            "0" => ch_sync::deadlock_example::run(),
            "1" => ch_sync::ch01_thread::run(),
            "2" => ch_sync::ch02_mutex::run(),
            "3-1" => ch_sync::ch03_1_condvar_wait::run(),
            "3-2" => ch_sync::ch03_2_condvar_wait_while::run(),
            "4" => ch_sync::ch04_rwlock::run(),
            "5" => ch_sync::ch05_barrier::run(),
            "6" => ch_sync::ch06_semaphore::run(),
            "7" => ch_sync::ch07_channel::run(),
            "8" => ch_sync::ch08_bakery::run(),
            "9" => ch_sync::ch09_rwlock::run(),
            "10" => ch_sync::ch10_banker::run(),
            _ => {
                println!("common 카테고리의 해당 번호가 없습니다.");
            }
        },

        _ => {
            println!("알 수 없는 category 입니다.");
        }
    }
}
