use std::{io::{stdout, Write}, thread::sleep, time::Duration};

const NUM:u128 = 5;

fn main(){
  print_msg("hello ");
  print_msg("world\n");
}

fn print_msg(s:&str){
  for _ in 0..NUM {
      print!("{}",s);
      let _ = stdout().flush();
      sleep(Duration::from_secs(1));
  }
}