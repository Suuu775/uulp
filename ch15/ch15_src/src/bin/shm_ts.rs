use std::{process::exit, ptr::null_mut};

use nix::{libc::{self, c_char, ctime_r, shmat, shmctl, shmget, IPC_CREAT, IPC_RMID}, unistd::sleep};

const TIME_MEM_KEY:i32 = 99;
const SEG_SIZE:usize = 100;

fn main(){
    let seg_id = unsafe { shmget(TIME_MEM_KEY, SEG_SIZE, IPC_CREAT|0o777) };
    if seg_id==-1 {
        eprintln!("shmget");
        exit(1);
    }

    let mem_ptr =  unsafe { shmat(seg_id, std::ptr::null(), 0)  as *mut c_char};
    if unsafe { *mem_ptr } == -1  {
      eprintln!("shmat");
      exit(2);
    }

    for _ in 0..60{
      let now:i64 = 0;
      unsafe { libc::time(now as *mut i64) };
      unsafe { ctime_r(now as *const i64, mem_ptr) };
      sleep(1);
    }

    unsafe { shmctl(seg_id, IPC_RMID, null_mut()) };
}