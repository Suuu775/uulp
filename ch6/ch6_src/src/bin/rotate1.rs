use std::{error::Error, io::{self, Read}, mem, process::exit};

use libc::{signal, tcgetattr, tcsetattr, termios, ECHO, ICANON, SIGINT, SIG_IGN, TCSANOW, VMIN};

fn main(){
    unsafe { signal(SIGINT, SIG_IGN) };
    tty_mode(0);
    set_mode();
    while let Ok(c) = getchar() {
        if c=='z' {
            eprint!("a");
        } else if c.is_lowercase() {
            eprint!("{}",(c as u8 +1) as char);
        } else if c=='Q' {
            tty_mode(1);
            exit(0);
        } else {
            eprint!("{}",c);
        }
    }
    tty_mode(1);
}

fn getchar()->Result<char,Box<dyn Error>>{
    let mut buffer = [0;1];
    let mut stdin = io::stdin().lock();
    stdin.read_exact(&mut buffer)?;
    let char = buffer[0] as char;
    Ok(char)
}

fn tty_mode(how: i32)->i32{
    static mut ORIGINAL_MODE:termios = unsafe {mem::zeroed()};
    if how==0 {
        unsafe { tcgetattr(0, &raw mut ORIGINAL_MODE as *mut termios) }
    } else {
        unsafe { tcsetattr(0, TCSANOW, &raw const ORIGINAL_MODE as *const termios) }
    }
}

fn set_mode(){
    let mut ttystate:termios = unsafe {mem::zeroed()};
    unsafe { tcgetattr(0, &raw mut ttystate as *mut termios) };
    ttystate.c_lflag &= !ICANON;
    ttystate.c_lflag &= !ECHO;
    ttystate.c_cc[VMIN] = 1;
    unsafe { tcsetattr(0, TCSANOW, &raw const ttystate as *const termios) };
}