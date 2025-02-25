use std::{io::{self}, process::{Command, Stdio}, os::unix::io::{FromRawFd, AsRawFd}};
use std::fs::File;
use nix::unistd::{pipe, fork, ForkResult, dup2, close};

#[allow(dead_code)]
fn popen(command: &str, mode: &str) -> Result<Box<dyn io::Read + Send>, Box<dyn std::error::Error>> {
    let (_parent_end, child_end) = match mode {
        "r" => (0, 1),
        "w" => (1, 0),
        _ => return Err("Invalid mode".into()),
    };

    let (read_fd, write_fd) = pipe()?;

    match unsafe { fork() }? {
        ForkResult::Parent { child: _ } => {
            // Parent process
            close(write_fd.as_raw_fd())?;
            let file = unsafe { File::from_raw_fd(read_fd.as_raw_fd()) };
            Ok(Box::new(file) as Box<dyn io::Read + Send>)
        }
        ForkResult::Child => {
            // Child process
            close(read_fd.as_raw_fd())?;
            dup2(write_fd.as_raw_fd(), child_end)?;
            close(write_fd.as_raw_fd())?;
            Command::new("/bin/sh")
                .arg("-c")
                .arg(command)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()?;
            std::process::exit(0);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}