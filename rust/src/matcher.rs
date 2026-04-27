use crate::common;

pub fn run(args: &[String]) {
    match args[1].as_str() {
        "common" => match args[2].as_str() {
            "1" => common::ch01_test::run(),
            _ => {
                println!("common 카테고리의 해당 번호가 없습니다.");
            }
        },

        _ => {
            println!("알 수 없는 category 입니다.");
        }
    }
}
