use std::{
    env::args,
    error::Error,
    process::{exit, Command, Stdio},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("Usage:pipen: [cmd]...");
        exit(1);
    }

    let mut previous_output = None;

    for (i, cmd) in args.iter().enumerate() {
        match i {
            0 => {
                previous_output = Some(
                    Command::new(cmd)
                        .stdout(Stdio::piped())
                        .spawn()?
                        .stdout
                        .expect(format!("can't get cmd{} stdout", i).as_str()),
                )
            }
            i if i == args.len() - 1 => {
                if let Some(previous_output) = previous_output.take() {
                    let res_output = Command::new(cmd).stdin(previous_output).output()?.stdout;
                    println!("{}", String::from_utf8(res_output).unwrap());
                }
            }
            i => {
                if let Some(pre_output) = previous_output.take() {
                    previous_output = Some(Command::new(cmd)
                    .stdin(pre_output)
                    .stdout(Stdio::piped())
                    .spawn()?
                    .stdout
                    .expect(format!("can't get cmd{} stdout", i).as_str()))
                }
            }
        }
    }
    Ok(())
}
