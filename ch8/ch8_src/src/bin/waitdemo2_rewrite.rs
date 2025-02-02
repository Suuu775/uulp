use libc::{signal, SIGCHLD};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult};
use std::process::exit;
use std::time::Duration;
use std::thread::sleep;

const DELAY: u64 = 10;

extern "C" fn handler(signum: i32) {
    if signum == SIGCHLD {
        println!("SIGCHLD received");
    }
}

fn main() {
    unsafe {
        signal(SIGCHLD, handler as usize);
    }
    println!("before: mypid is {}", std::process::id());

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            parent_code(child);
        }
        Ok(ForkResult::Child) => {
            child_code(DELAY);
        }
        Err(err) => {
            eprintln!("fork failed: {:?}", err);
        }
    }
}

fn child_code(delay: u64) {
    println!("child {} here, will sleep {} seconds", std::process::id(), delay);
    sleep(Duration::from_secs(delay));
    println!("child done, about to exit");
    exit(17);
}

fn parent_code(childpid: nix::unistd::Pid) {
    loop {
        println!("waiting");
        sleep(Duration::from_secs(1));

        match waitpid(childpid, None) {
            Ok(WaitStatus::Exited(pid, status)) => {
                println!("done waiting for {}. Wait returned status {}", pid, status);
                let high_8 = status >> 8;
                let low_7 = status & 0x7F;
                let bit_7 = (status & 0x80) != 0;

                println!("status: exit={}, sig={}, core={}", high_8, low_7, bit_7);
                break;
            }
            Ok(WaitStatus::Signaled(pid, signal, _)) => {
                println!("child {} was terminated by signal {:?}", pid, signal);
                break;
            }
            Ok(_) => {
                println!("child exited unexpectedly");
                break;
            }
            Err(err) => {
                eprintln!("wait failed: {:?}", err);
                break;
            }
        }
    }
}