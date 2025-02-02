use std::error::Error;

use nix::unistd::{fork, getpid, ForkResult};

fn main()->Result<(),Box<dyn Error>>{
    println!("Before my pid is {}\n",getpid());

    let fork_rv = unsafe { fork() }?;
    
    match fork_rv {
        ForkResult::Parent {  child } =>{
            println!("I am the parent,my chlid id is {}",child);
        },
        ForkResult::Child =>{
            println!("I am the chlid, my pid is {}",getpid());
        }
    }
    Ok(())
}