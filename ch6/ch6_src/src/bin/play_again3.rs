use libc::{tcgetattr, tcsetattr, termios, fcntl, O_NDELAY, F_GETFL, F_SETFL, ECHO, ICANON, TCSANOW, VMIN};
use std::io::{self, Read, Write};
use std::process;
use std::time::Duration;
use std::thread::sleep;

const QUESTION: &str = "Do you want another translation";
const TRIES: i32 = 3;
const SLEEPTIME: u64 = 2;
const BEEP: u8 = 7;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tty_mode(0)?;
    set_cr_noecho_mode()?;
    set_nodelay_mode()?;

    let response = get_response(QUESTION, TRIES)?;
    tty_mode(1)?;

    process::exit(response as i32);
}

fn tty_mode(how: i32) -> Result<(), Box<dyn std::error::Error>> {
    static mut ORIGINAL_MODE: termios = unsafe { std::mem::zeroed() };
    static mut ORIGINAL_FLAGS: i32 = 0;

    if how == 0 {
        unsafe {
            if tcgetattr(0, &raw mut ORIGINAL_MODE as *mut termios) != 0 {
                return Err("Failed to get terminal attributes".into());
            }
            ORIGINAL_FLAGS = fcntl(0, F_GETFL);
        }
    } else {
        unsafe {
            if tcsetattr(0, TCSANOW, &raw const ORIGINAL_MODE as *const termios) != 0 {
                return Err("Failed to set terminal attributes".into());
            }
            fcntl(0, F_SETFL, ORIGINAL_FLAGS);
        }
    }

    Ok(())
}

fn set_nodelay_mode() -> Result<(), Box<dyn std::error::Error>> {
    let flags = unsafe { fcntl(0, F_GETFL) };
    if flags < 0 {
        return Err("Failed to get file descriptor flags".into());
    }

    let new_flags = flags | O_NDELAY;
    unsafe {
        if fcntl(0, F_SETFL, new_flags) != 0 {
            return Err("Failed to set file descriptor flags".into());
        }
    }

    Ok(())
}

fn set_cr_noecho_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut ttystate: termios = unsafe { std::mem::zeroed() };

    unsafe {
        if tcgetattr(0, &mut ttystate) != 0 {
            return Err("Failed to get terminal attributes".into());
        }
    }

    ttystate.c_lflag &= !ICANON;
    ttystate.c_lflag &= !ECHO;
    ttystate.c_cc[VMIN as usize] = 1;

    unsafe {
        if tcsetattr(0, TCSANOW, &ttystate) != 0 {
            return Err("Failed to set terminal attributes".into());
        }
    }

    Ok(())
}

fn get_ok_char() -> Result<char, Box<dyn std::error::Error>> {
    loop {
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer)?;
        let c = buffer[0] as char;

        if c.to_ascii_lowercase() == 'y' || c.to_ascii_lowercase() == 'n' {
            return Ok(c);
        }
    }
}

fn get_response(question: &str, maxtries: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let _ = maxtries;
    print!("{} (y/n)? ", question);
    io::stdout().flush()?;

    for _ in 0..TRIES {
        sleep(Duration::from_secs(SLEEPTIME));
        let input = get_ok_char()?;

        match input.to_ascii_lowercase() {
            'y' => return Ok(0),
            'n' => return Ok(1),
            _ => {
                print!("{}", BEEP as char);
                continue;
            }
        }
    }
    Ok(2)
}