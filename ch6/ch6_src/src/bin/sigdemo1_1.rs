use std::{error::Error, io::{self, Read}, process::exit, thread::sleep, time::Duration};

use libc::{sighandler_t, signal, SIGINT};

fn main() {
    unsafe {
        signal(SIGINT, f as sighandler_t);
    }
    for i in 1..=6 {
        if i%3==0 {
            println!("Interrupted! OK to quit (y/n)?"); 
            if let Ok(ch) = getchar() {
                sleep(Duration::from_secs(1));
                match ch {
                    'y' | 'Y' => exit(0),
                    'n' | 'N' => (),
                    _ => {
                        exit(1);
                    }
                }
            }
        } else {
            println!("hello");
        }
        sleep(Duration::from_secs(1));
    }
}

extern "C" fn f(_signum: i32) {
    println!("OOCH");
}

fn getchar()->Result<char,Box<dyn Error>>{
    let mut buffer = [0;1];
    let mut stdin = io::stdin().lock();
    stdin.read_exact(&mut buffer)?;
    let char = buffer[0] as char;
    Ok(char)
}