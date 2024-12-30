use std::{error::Error, fs::rename, path::{Path, PathBuf}, process::exit};

use clap::Parser;
#[derive(Parser)]
struct Cli{
    source:String,
    dest:String
}


// simply mv
fn main()->Result<(),Box<dyn Error>>{
    let cli = Cli::parse();
    let source = Path::new(&cli.source);
    let dest = Path::new(&cli.dest);
    
    if !source.is_file() {
        eprintln!("mv: cannot stat '{}': No such file or directory",source.as_os_str().to_string_lossy());
        exit(1);
    }
    
    if !dest.is_dir() {
        rename(source, dest)?;
    }

    if dest.is_dir() {
        let mut dest = PathBuf::from(dest);
        if let Some(source) = source.file_name() {
            dest.push(source);
        }else {
            eprintln!("Source has no file name");
            exit(1)
        }
        rename(source, dest)?;
    }

    Ok(())
}