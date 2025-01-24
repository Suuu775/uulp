use std::{env::args, error::Error, process::exit, sync::{LazyLock, RwLock}};

use libc::{sighandler_t, signal, SIGINT};

static EXPECT_NUM:LazyLock<u32> = LazyLock::new(||{
    let args:Vec<String> = args().skip(1).collect();
    args[0].parse::<u32>().unwrap()
});

static ACTUAL_NUM:RwLock<u32> = RwLock::new(0);

static OUCH:LazyLock<RwLock<String>> = LazyLock::new(||{RwLock::new("OUCH".to_string())});

fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String> = args().skip(1).collect();
    if args.len()!=1 {
        eprintln!("args nums error");
        exit(1);
    }

    unsafe { signal(SIGINT, handler as sighandler_t) };
    loop {
        
    }
}


extern "C" fn handler(_signum:i32){
    if let Ok(mut actual_num) = ACTUAL_NUM.write() {
        *actual_num+=1;

        if *actual_num==*EXPECT_NUM {
            modify_ouch();
            exit(0);
        } else {
            modify_ouch();
        }
    } else {
        eprintln!("can't get ACTUAL_NUM write");
        exit(1);
    }
}


fn modify_ouch(){
        if let Ok(mut ouch) = OUCH.write() {
            ouch.push('!');
            println!("{}",ouch);
        } else {
            eprintln!("can't get OUCH write");
            exit(1)
        }
}