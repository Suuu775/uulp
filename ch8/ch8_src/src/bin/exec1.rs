use std::{error::Error, ffi::CString};

use nix::unistd::execvp;

fn main() -> Result<(), Box<dyn Error>> {
    let arglist: Vec<CString> = vec!["ls", "-l"]
        .into_iter()
        .map(|s| CString::new(s).unwrap())
        .collect();

    println!("*** about to exec ls -l");

    let filename = CString::new("ls")?;
    execvp(&filename, &arglist)?;

    println!("*** ls is done.bye");
    Ok(())
}
