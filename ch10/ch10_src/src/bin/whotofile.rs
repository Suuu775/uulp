use nix::libc::{c_char, creat};
use nix::unistd::{close, dup2, execvp, fork, ForkResult};
use nix::sys::wait::wait;
use std::error::Error;
use std::ffi::CString;

// WARNING!!! It can't work
fn main() -> Result<(), Box<dyn Error>> {
    println!("About to run who into a file");

    match unsafe { fork()? } {
        ForkResult::Parent {  .. } => {
            wait()?;
            println!("Done running who. Results in userlist");
        }
        ForkResult::Child => {
            let _ =  dup2(-1, 1) ; // Close standard output by duplicating to an invalid fd
            let fd = unsafe { creat("userlist".as_ptr() as *const u8 as *const c_char, 0o644) };
            let _ = dup2(fd, 1); // Redirect standard output to the file
            let _ = close(fd);
            let who_args = vec![CString::new("who").unwrap()];
            execvp(&CString::new("who").unwrap(), &who_args)?;
            println!("execvp failed");
        }
    }

    Ok(())
}