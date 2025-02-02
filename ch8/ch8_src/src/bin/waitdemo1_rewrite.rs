// waitdemo1_rewrite.rs: Instead of using processes, threads are used

use std::{env::args, error::Error, process::exit, thread::{self, sleep}, time::Duration};

use rand::Rng;
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String> = args().skip(1).collect();
    if args.len()!=1||args[0].parse::<u32>().is_err() {
        eprintln!("the arg is incorrect");
        exit(1);
    }
    let mut thread_handles = vec![];
    let thread_num = args[0].parse::<u32>()?;
    for _ in 0..thread_num {
        thread_handles.push(
            thread::spawn(||{
                let seconds = rand::rng().random_range(1..=2);
                sleep(Duration::from_secs(seconds));
                println!("I spend {} seconds",seconds);
            })
        );
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }
    Ok(())
}