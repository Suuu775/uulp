use std::{error::Error, fs::File, io::{stdin, stdout, Read, Write}, os::fd::FromRawFd};

use nix::libc::{c_char, close, open, O_RDONLY};

// WARNING!!! It can't work
fn main()->Result<(),Box<dyn Error>>{
    let mut line = String::new();

    // raed and print three lines
    for _ in 0..3 {
        stdin().read_line(&mut line)?;
        println!("{}",line);
        stdout().flush()?;
        line.clear();
    }

    // redirect input to file fd
    unsafe { close(0) };
    let fd = unsafe { open("/etc/passwd".as_ptr() as *const c_char, O_RDONLY) };
    if fd != 0 {
        eprintln!("Could not open /etc/passwd as fd 0");
        std::process::exit(1);
    }

    // raed and print three lines
    let mut line = [0;100];
    let mut stdin = unsafe { File::from_raw_fd(0) };
    for _ in 0..3 {
        stdin.read_exact(&mut line)?;
        print!("{}", String::from_utf8_lossy(&line));
        stdout().flush()?;
    }

    Ok(())
}