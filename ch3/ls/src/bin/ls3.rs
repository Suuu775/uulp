use std::{error::Error, path::Path, process::exit};
use clap::Parser;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[arg(short = 'R')]
    rec:bool,
    file:Vec<String>
}


fn main()->Result<(),Box<dyn Error>>{
    let cli = Cli::parse();
    let filevec = cli.file;
    match filevec.len() {
        0 => return do_ls_rec(Path::new("."),false),
        1 =>{
            for filepath in filevec {
                let filepath = Path::new(&filepath);
                if filepath.is_file() {
                    print!("{}  ",filepath.to_string_lossy());
                } else if filepath.is_dir() {
                    return do_ls_rec(Path::new(filepath),true);
                } else {
                    println!("ls: cannot access '{}': No such file or directory",filepath.to_string_lossy());
                    exit(1)
                }
            }
        },
        _ =>{
            for filepath in filevec {
                let filepath = Path::new(&filepath);
                if filepath.is_file() {
                    print!("{}  ",filepath.to_string_lossy());
                } else if filepath.is_dir() {
                    return do_ls_rec(Path::new(filepath),false);
                } else {
                    println!("ls: cannot access '{}': No such file or directory",filepath.to_string_lossy());
                    exit(1)
                }
            }
        }
    }
    Ok(())
}

fn do_ls_rec(path:&Path,signton:bool)->Result<(),Box<dyn Error>>{
    if signton==false {
        println!("{}:",path.to_string_lossy());
    }
    for direntry in path.read_dir()? {
        let direntry = direntry?;
        let entrytype = direntry.file_type()?;
        if entrytype.is_dir() {
            print!("{} ",direntry.file_name().to_string_lossy());
             do_ls_rec(&direntry.path(),false)?
        } else {
            print!("{} ",direntry.file_name().to_string_lossy());
        }
    }
    Ok(())
}