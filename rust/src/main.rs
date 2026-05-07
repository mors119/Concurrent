use std::env;

mod ch_sync;
mod matcher;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("사용법: cargo run <category> <number> [extra args...]");
        return;
    }

    matcher::run(&args);
}
