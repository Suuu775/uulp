use std::{env, error::Error, path::Path, u32};

use nix::unistd::{chown, User};

fn main() ->Result<(),Box<dyn Error>>{
    let args:Vec<String> = env::args().skip(1).collect();
    let owner = &args[0];
    let files_path:Vec<&Path> = args[1..].iter().map(|s|{
        Path::new(s)
    }).collect();
    if owner.as_bytes().iter().all(|&ch|{ch.is_ascii_digit()}) {
        let uid = owner.parse::<u32>()?;
        return chown_all_file(uid, files_path);
    } else {
        let user = User::from_name(&owner)?;
        if let Some(user) = user {
            return chown_all_file(user.uid.into(), files_path);
        } else {
            eprintln!("chown: invalid user:{}",owner);
        }
    }
    Ok(())
}

fn chown_all_file(uid:u32,files_path:Vec<&Path>)->Result<(),Box<dyn Error>>{
    for file_path in files_path {
        chown(file_path, Some(uid.into()), None)?
    }
    Ok(())
}