use std::{
    env::args,
    error::Error,
    fs::read_to_string,
    process::exit,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let total_word = Arc::new(Mutex::new(0));

    let args: Vec<String> = args().collect();
    let file1 = args[1].clone();
    let file2 = args[1].clone();

    if args.len() != 3 {
        eprintln!("usage {} file1 file", args[0]);
        exit(1);
    }

    let total_clone1 = Arc::clone(&total_word);
    let t1 = thread::spawn(move || {
        let _ = count_word(&file1, total_clone1);
    });

    let total_clone2 = Arc::clone(&total_word);
    let t2 = thread::spawn(move || {
        let _ = count_word(&file2, total_clone2);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let total = total_word.lock().unwrap();
    println!("{:5}: total words", *total);
}

fn count_word(filename: &String, total_word: Arc<Mutex<i32>>) -> Result<(), Box<dyn Error>> {
    let content = read_to_string(filename)?;
    for _ in content.split_whitespace() {
        let mut total_word = total_word.lock().unwrap();
        *total_word += 1;
    }
    Ok(())
}
