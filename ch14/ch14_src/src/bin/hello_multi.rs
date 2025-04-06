use std::{io::{stdout, Write}, thread::{self, sleep}, time::Duration};

const NUM:u128 = 5;

fn main(){
  let t1 = thread::spawn(||{print_msg("hello ");});
  let t2 = thread::spawn(||{print_msg("world\n");});
  t1.join().unwrap();
  t2.join().unwrap();
}

fn print_msg(s:&str){
  for _ in 0..NUM {
      print!("{}",s);
      let _ = stdout().flush();
      sleep(Duration::from_secs(1));
  }
}