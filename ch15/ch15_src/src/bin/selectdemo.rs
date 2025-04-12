use std::{env::args, fs::File, io::{self, Read, Write}, os::fd::AsFd};

use nix::sys::{select::{select, FdSet}, time::{TimeVal, TimeValLike}};

fn main()->io::Result<()>{
  let args:Vec<String> = args().collect();
  if args.len()!=4 {
    eprintln!("usage: {} file1 file2 timeout", args[0]);
    std::process::exit(1);
  }

  let file1 = File::open(&args[1]).expect("Failed to open file1");
  let fd1 = file1.as_fd();
  let file2 = File::open(&args[2]).expect("Fail to open file2");
  let fd2 = file2.as_fd();

  let timeout_seconds = args[3].parse::<i32>().expect("Fail to parse timeout format");
  loop {
      let mut readfds = FdSet::new();
      readfds.insert(fd1);
      readfds.insert(fd2);
      let mut timeout = TimeVal::seconds(timeout_seconds.into());

      match select(None, &mut readfds, None, None,&mut timeout) {
        Ok(retval) => {
          if retval > 0 {
              if readfds.contains(fd1) {
                  showdata(&args[1], file1.try_clone()?)?;
              }
              if readfds.contains(fd2) {
                  showdata(&args[2], file2.try_clone()?)?;
              }
          } else {
              println!("no input after {} seconds", timeout_seconds);
          }
      }
      Err(err) => {
          eprintln!("select error: {}", err);
          std::process::exit(4);
      }
  }
  }
}

fn showdata(fname: &str, mut file: File) -> io::Result<()> {
  let mut buf = [0u8; 4096];
  let n = file.read(&mut buf)?;

  if n == 0 {
      return Ok(());
  }

  print!("{}: ", fname);
  io::stdout().flush()?;
  io::stdout().write_all(&buf[..n])?;
  println!();
  Ok(())
}