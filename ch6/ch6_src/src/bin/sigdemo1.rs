use std::{thread::sleep, time::Duration};

use libc::{sighandler_t, signal, SIGINT};

fn main() {
    unsafe {
        signal(SIGINT, f as sighandler_t);
    }
    for _ in 0..5 {
        println!("hello");
        sleep(Duration::from_secs(1));
    }
}

extern "C" fn f(_signum: i32) {
    println!("OOCH");
}
