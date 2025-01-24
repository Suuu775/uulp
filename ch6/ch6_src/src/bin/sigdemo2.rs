use std::{thread::sleep, time::Duration};

use libc::{signal, SIGINT, SIG_IGN};

fn main() {
    unsafe {
        signal(SIGINT, SIG_IGN);
    }
    println!("you can't stop me!");
    loop {
        sleep(Duration::from_secs(1));
        println!("jiejiejieje");
    }
}
