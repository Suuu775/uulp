use nix::unistd::{close, dup2, fork, pipe, ForkResult};
use nix::{libc::execlp, sys::wait::wait};
use std::fs::File;
use std::io::BufRead;
use std::os::fd::OwnedFd;
use std::{
    error::Error,
    io::{self, Read, Write},
    os::{fd::FromRawFd, unix::io::AsRawFd},
};

// can't success run it
fn main() -> Result<(), Box<dyn Error>> {
    let todc: [OwnedFd; 2] = pipe()?.into();
    let fromdc: [OwnedFd; 2] = pipe()?.into();

    match unsafe { fork() }? {
        ForkResult::Parent { child: _ } => {
            be_bc(todc, fromdc)?;
            wait()?;
        }
        ForkResult::Child => {
            be_dc(todc, fromdc)?;
        }
    }

    Ok(())
}

fn be_dc(todc: [OwnedFd; 2], fromdc: [OwnedFd; 2]) -> Result<(), Box<dyn Error>> {
    dup2(todc[0].as_raw_fd(), 0)?;
    close(todc[0].as_raw_fd())?;
    close(todc[1].as_raw_fd())?;

    dup2(fromdc[1].as_raw_fd(), 1)?;
    close(fromdc[1].as_raw_fd())?;
    close(fromdc[0].as_raw_fd())?;

    unsafe { execlp("dc".as_ptr() as *const i8, "dc".as_ptr() as *const i8) };
    Err("Can't run dc".into())
}

fn be_bc(todc: [OwnedFd; 2], fromdc: [OwnedFd; 2]) -> Result<(), Box<dyn Error>> {
    close(todc[0].as_raw_fd())?;
    close(fromdc[1].as_raw_fd())?;

    let mut todc_file = unsafe { File::from_raw_fd(todc[1].as_raw_fd()) };
    let mut fromdc_file = unsafe { File::from_raw_fd(fromdc[0].as_raw_fd()) };

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    loop {
        print!("tinybc: ");
        io::stdout().flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 3 {
            println!("syntax error");
            continue;
        }

        let num1: i32 = parts[0].parse()?;
        let operation = parts[1].chars().next().unwrap();
        let num2: i32 = parts[2].parse()?;

        let command = format!("{}\n{}\n{}\np\n", num1, num2, operation);
        todc_file.write_all(command.as_bytes())?;
        todc_file.flush()?;

        let mut result = String::new();
        fromdc_file.read_to_string(&mut result)?;
        println!("{} {} {} = {}", num1, operation, num2, result.trim());
    }
}
