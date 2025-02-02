use std::{error::Error, ffi::CString, process::exit,io};

use nix::unistd::execvp;

const ARGLEN:usize = 100;
const MAXARGS:usize = 20;

fn main(){
    let mut numargs:usize = 0;
    let mut argbuf = String::new();
    let mut arglist:Vec<String> = Vec::with_capacity(ARGLEN);
    while numargs<MAXARGS {
        eprint!("Arg[{}]?",numargs);
        if io::stdin().read_line(&mut argbuf).is_ok()&&argbuf!='\n'.to_string() {
            argbuf = argbuf.trim().to_string();
            arglist.push(argbuf.clone());
            numargs+=1;
            argbuf.clear();
        } else {
            if numargs>0 {
                let _ = execute(arglist[..numargs].to_vec());
                numargs = 0;
            }
        }

    }
}

fn execute(arglist:Vec<String>)->Result<(),Box<dyn Error>>{
    let arglist: Vec<CString> = arglist
        .into_iter()
        .map(|s| CString::new(s).unwrap())
        .collect();
    execvp(&arglist[0], &arglist)?;
    exit(1);
}