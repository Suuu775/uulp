use std::{env::args, error::Error, fs::{metadata, File}, path::Path, process::Command};

fn main()->Result<(),Box<dyn Error>>{

    // argument handle
    let args:Vec<String> = args().skip(1).collect();
    let file_path = Path::new(&args[0]);
    let file_metadata = metadata(file_path)?;
    if args.len()!=1{
        eprintln!("Usage:sortfromfile file");
    }
    if !file_metadata.is_file() {
        eprintln!("the arguement is not file");
    }

    // run sort program and change stdin to file
    let output = Command::new("sort")
    .stdin(File::open(&args[0]).unwrap())
    .output()
    .expect("can't run sort");

    // stdout result
    println!("{}",String::from_utf8(output.stdout).unwrap());
    Ok(())
}