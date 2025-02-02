use std::error::Error;

use nix::unistd::{fork, getpid};

// 2^(fork function num)
fn main()->Result<(),Box<dyn Error>>{
    // println!("my pid is {}",getpid());

    unsafe { fork()? };
    unsafe { fork()? };
    unsafe { fork()? };
    
    println!("my pid is {}",getpid());
    Ok(())
}