use libc::{fcntl, tcgetattr, tcsetattr, termios, ECHO, F_GETFL, F_SETFL, ICANON, O_NDELAY, TCSANOW, VMIN, VTIME};
use std::io::{self, Read, Write};
use std::process;
use std::time::Duration;
use std::thread::sleep;

const QUESTION: &str = "Do you want another translation";
const TRIES: i32 = 3;
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
            tcsetattr(0, TCSANOW, &raw const ORIGINAL_MODE as *const termios);
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
    ttystate.c_cc[VTIME as usize] = 20;

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
        match io::stdin().read_exact(&mut buffer) {
            Ok(_) => {
                let c = buffer[0] as char;
                if c.to_ascii_lowercase() == 'y' || c.to_ascii_lowercase() == 'n' {
                    return Ok(c);
                }
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }
}

fn get_response(question: &str, _maxtries: i32) -> Result<i32, Box<dyn std::error::Error>> {
    print!("{} (y/n)? ", question);
    io::stdout().flush()?;

    for _ in 0..TRIES {
        let input = get_ok_char()?;

        match input.to_ascii_lowercase() {
            'y' => return Ok(0),
            'n' => return Ok(1),
            _ => {
                print!("{}", BEEP as char);
                io::stdout().flush()?;
                continue;
            }
        }
    }
    Ok(2)
}