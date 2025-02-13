use std::{
    env::args, error::Error, process::{exit, Command, Stdio}
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().skip(1).collect();
    if args.len() != 3 {
        eprintln!("Usage:pipe3: cmd1 cmd2 cmd3");
        exit(1);
    }

    let cmd1_output = Command::new(&args[0])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .expect("can't get cmd1 stdout");
    let cmd2_output = Command::new(&args[1])
        .stdin(cmd1_output)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .expect("can't get cmd2 stdout");
    let cmd3_output = Command::new(&args[2])
        .stdin(cmd2_output)
        .output()?
        .stdout;
    println!("{}",String::from_utf8(cmd3_output).unwrap());
    Ok(())
}
