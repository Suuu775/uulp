use std::{error::Error, mem::MaybeUninit, process::exit, sync::RwLock};
use std::mem::size_of;
use libc::{c_void, ioctl, read, signal, tcgetattr, tcsetattr, termios, winsize, ECHO, ICANON, SIGWINCH, TCSANOW, TIOCGWINSZ, VMIN};

static MODIFY_CHAR: RwLock<char> = RwLock::new('A');

fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        signal(SIGWINCH, handler as usize);
    }
    tty_mode(0);
    let mut input_event: char = '0';
    set_crmode();
    loop {
        monitor_key(&mut input_event);
        eprintln!("{}", input_event);
        if input_event == 'Q' {
            print!("{}[2J", 27 as char);
            tty_mode(1);
            exit(0)
        } else {
            if let Ok(mut modify_char) = MODIFY_CHAR.write() {
                *modify_char = 'A';
            } else {
                eprintln!("can't get modify_char write");
                tty_mode(1);
                exit(1);
            }
            rotate_char();
        }
    }
}

extern "C" fn handler(_signum: i32) {
    rotate_char();
}

fn screen_dimensions() -> (u16, u16) {
    let wbuf: winsize = unsafe { MaybeUninit::zeroed().assume_init() };
    if unsafe { ioctl(0, TIOCGWINSZ, &wbuf) } != -1 {
        return (wbuf.ws_row, wbuf.ws_col);
    } else {
        unreachable!()
    }
}

fn monitor_key(input_event: &mut char) {
    let bytes_read = unsafe {
        read(
            0,
            input_event as *mut char as *mut c_void,
            size_of::<char>(),
        )
    };

    if bytes_read < 0 {
        eprintln!("Failed to read input event");
        tty_mode(1);
        exit(1);
    }
}

fn rotate_char() {
    let (row, col) = screen_dimensions();
    if let Ok(mut modify_char) = MODIFY_CHAR.write() {
        let s = modify_char.to_string().repeat(col.into());
        for _ in 0..row {
            println!("{}", s);
        }

        if *modify_char == 'Z' {
            *modify_char = 'A';
        } else {
            *modify_char = (*modify_char as u8 + 1) as char;
        }
    } else {
        eprintln!("can't get modify_char write");
        tty_mode(1);
        exit(1);
    }
}

fn set_crmode(){
    let mut ttystate:termios = unsafe {MaybeUninit::zeroed().assume_init()};
    unsafe { tcgetattr(0, &raw mut ttystate as *mut termios) };
    ttystate.c_lflag &= !ICANON;
    ttystate.c_lflag &= !ECHO;
    ttystate.c_cc[VMIN] = 1;
    unsafe { tcsetattr(0, TCSANOW, &raw const ttystate as *const termios) };
}

fn tty_mode(how: i32)->i32{
    static mut ORIGINAL_MODE:termios = unsafe {MaybeUninit::zeroed().assume_init()};
    if how==0 {
        unsafe { tcgetattr(0, &raw mut ORIGINAL_MODE as *mut termios) }
    } else {
        unsafe { tcsetattr(0, TCSANOW, &raw const ORIGINAL_MODE as *const termios) }
    }
}