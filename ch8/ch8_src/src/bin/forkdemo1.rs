use std::{error::Error, thread::sleep, time::Duration};

use nix::unistd::{fork, getpid};

fn main()->Result<(),Box<dyn Error>>{
    let mypid = getpid();
    println!("Before: my pid is {}",mypid);

    let ret_from_fork = unsafe { fork() }?;

    sleep(Duration::from_secs(1));
    println!("After: my pid is {},fork() said is {:?}",getpid(),ret_from_fork); 
    Ok(())
}