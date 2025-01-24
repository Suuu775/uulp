use std::{mem, thread::sleep, time::Duration};

use libc::{tcgetattr, tcsetattr, termios, ISIG, TCSANOW};

fn main() {
    println!("you can't stop me!");
    set_crmode();
    loop {
        sleep(Duration::from_secs(1));
        println!("jiejiejieje");
    }
}

fn set_crmode(){
    let mut ttystate:termios = unsafe {mem::zeroed()};
    unsafe { tcgetattr(0, &raw mut ttystate as *mut termios) };
    ttystate.c_lflag &= !ISIG;
    unsafe { tcsetattr(0, TCSANOW, &raw const ttystate as *const termios) };
}