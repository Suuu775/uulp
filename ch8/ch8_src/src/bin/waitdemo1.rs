use nix::sys::wait::{wait, WaitStatus};
use nix::unistd::{fork, ForkResult};
use std::process::exit;
use std::time::Duration;
use std::thread::sleep;

const DELAY: u64 = 2;

fn main() {
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

fn parent_code(_childpid: nix::unistd::Pid) {
    match wait() {
        Ok(WaitStatus::Exited(pid, status)) => {
            println!("done waiting for {}. Wait returned status {}", pid, status);
        }
        Ok(WaitStatus::Signaled(pid, signal, _)) => {
            println!("child {} was terminated by signal {:?}", pid, signal);
        }
        Ok(_) => {
            println!("child exited unexpectedly");
        }
        Err(err) => {
            eprintln!("wait failed: {:?}", err);
        }
    }
}