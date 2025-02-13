use std::{env::args, error::Error, process::{Command, Stdio}};

fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String> = args().skip(1).collect();
    let output = Command::new(&args[0])
    .stdin(Stdio::inherit())
    .stdout(Stdio::piped())
    .args(&args[1..])
    .output()?;

    print!("{}",String::from_utf8_lossy(&output.stdout));
    Ok(())
}