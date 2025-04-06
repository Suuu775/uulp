use std::{
  fs::File,
  io::{self, Read},
  path::Path,
  sync::{Arc, Condvar, Mutex},
  thread,
};

struct ArgSet {
  fname: String,
  count: usize,
}

struct SharedState {
  mailbox: Mutex<Option<Arc<ArgSet>>>,
  condvar: Condvar,
}

fn main() -> io::Result<()> {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 3 {
      eprintln!("Usage: {} file1 file2", args[0]);
      std::process::exit(1);
  }

  let shared = Arc::new(SharedState {
      mailbox: Mutex::new(None),
      condvar: Condvar::new(),
  });

  let mut handles = vec![];
  for filename in &args[1..=2] {
      let shared_clone = Arc::clone(&shared);
      let filename = filename.clone();
      
      let handle = thread::spawn(move || {
          let count = match count_words(&filename) {
              Ok(n) => n,
              Err(e) => {
                  eprintln!("Error processing {}: {}", filename, e);
                  0
              }
          };
          
          let result = Arc::new(ArgSet {
              fname: filename,
              count,
          });

          let mut mailbox = shared_clone.mailbox.lock().unwrap();
          while mailbox.is_some() {
              mailbox = shared_clone.condvar.wait(mailbox).unwrap();
          }
          
          *mailbox = Some(result);
          shared_clone.condvar.notify_one();
      });
      
      handles.push(handle);
  }


  let mut total_words = 0;
  let mut reports_received = 0;
  
  let mut mailbox = shared.mailbox.lock().unwrap();
  while reports_received < 2 {
      println!("MAIN: waiting for notification");

      mailbox = shared.condvar.wait(mailbox).unwrap();
      
      if let Some(result) = mailbox.take() {
          println!("MAIN: Received result for {}", result.fname);
          println!("{:7}: {}", result.count, result.fname);
          total_words += result.count;
          reports_received += 1;
          

          shared.condvar.notify_one();
      }
  }

  println!("{:7}: total words", total_words);
  

  for handle in handles {
      handle.join().unwrap();
  }

  Ok(())
}

fn count_words(filename: &str) -> io::Result<usize> {
  let mut file = File::open(Path::new(filename))?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let mut count = 0;
  let mut in_word = false;
  
  for c in contents.chars() {
      if c.is_ascii_alphanumeric() {
          if !in_word {
              count += 1;
              in_word = true;
          }
      } else {
          in_word = false;
      }
  }

  Ok(count)
}