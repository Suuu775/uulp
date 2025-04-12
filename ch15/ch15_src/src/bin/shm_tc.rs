use std::{ffi::c_char, os::raw::c_void, process::exit};

use nix::libc::{shmat, shmdt, shmget};

const TIME_MEM_KEY: i32 = 99;
const SEG_SIZE: usize = 100;

fn main() {
    let seg_id = unsafe { shmget(TIME_MEM_KEY, SEG_SIZE, 0o777) };
    if seg_id == -1 {
        eprintln!("shmget");
        exit(1);
    }

    let mem_ptr =  unsafe { shmat(seg_id, std::ptr::null(), 0)  as *mut c_char};
    if unsafe { *mem_ptr } == -1  {
      eprintln!("shmat");
      exit(2);
    }
    let mut buffer = [0u8; SEG_SIZE];
    for i in 0..SEG_SIZE {
        let c = unsafe { *mem_ptr.add(i) };
        if c == 0 {
            break;
        }
        buffer[i] = c as u8;
    }

    let content = match std::str::from_utf8(&buffer) {
        Ok(s) => s.trim_matches('\0'), 
        Err(e) => {
            eprintln!("Error converting to string: {}", e);
            exit(3);
        }
    };

    println!("The time,direct from memory:...{}", content);
    unsafe { shmdt(mem_ptr as *const c_void) };
}
