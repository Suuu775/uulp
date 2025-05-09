use std::{error::Error, io::{self, Read}, mem};

use libc::{c_int, tcgetattr, tcsetattr, termios, ICANON, TCSANOW, VMIN};

const QUESTION:&str = "Do you want another translation";
fn main()->Result<(),Box<dyn Error>>{
    tty_mode(0);
    set_crmode();
    let _response = get_response(QUESTION)?;
    tty_mode(1);
    Ok(())
}


fn tty_mode(how: i32)->c_int{
    static mut ORIGINAL_MODE:termios = unsafe {mem::zeroed()};
    if how==0 {
        unsafe { tcgetattr(0, &raw mut ORIGINAL_MODE as *mut termios) }
    } else {
        unsafe { tcsetattr(0, TCSANOW, &raw const ORIGINAL_MODE as *const termios) }
    }
}

fn set_crmode(){
    let mut ttystate:termios = unsafe {mem::zeroed()};
    unsafe { tcgetattr(0, &raw mut ttystate as *mut termios) };
    ttystate.c_lflag &= !ICANON;
    ttystate.c_cc[VMIN] = 1;
    unsafe { tcsetattr(0, TCSANOW, &raw const ttystate as *const termios) };
}

fn getchar()->Result<char,Box<dyn Error>>{
    let mut buffer = [0;1];
    let mut stdin = io::stdin().lock();
    stdin.read_exact(&mut buffer)?;
    let char = buffer[0] as char;
    Ok(char)
}

fn get_response(question: &str) -> Result<i32,Box<dyn Error>> {
    println!("{} (y/n)?", question);

    loop {
        let input = getchar()?;

        match input {
            'y' | 'Y' => return Ok(0),
            'n' | 'N' => return Ok(1),
            _ => {
                println!("\ncan't understand {}", input as u8 as char);
                println!("please type y or n");
            }
        }
    }
}