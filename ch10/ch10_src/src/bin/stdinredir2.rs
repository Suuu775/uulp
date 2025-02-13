use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, dup2};
use std::io::{self, Read, Write};
use std::os::unix::io::FromRawFd;
use std::fs::File;

// WARNING!!! It can't work
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = [0u8; 100];

    // Read and print three lines from stdin
    for _ in 0..3 {
        io::stdin().read_exact(&mut line)?;
        print!("{}", String::from_utf8_lossy(&line));
        io::stdout().flush()?;
    }

    // Open /etc/passwd and redirect input to file fd
    let fd = open("/etc/passwd", OFlag::O_RDONLY, Mode::empty())?;
    let _newfd = dup2(fd, 0)?;
    close(fd)?;

    // Read and print three lines from the new stdin
    let mut stdin = unsafe { File::from_raw_fd(0) };
    for _ in 0..3 {
        stdin.read_exact(&mut line)?;
        print!("{}", String::from_utf8_lossy(&line));
        io::stdout().flush()?;
    }

    Ok(())
}