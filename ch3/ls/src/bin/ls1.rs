use std::{env, error::Error, fs};

fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 =>{
            do_ls(".",false)
        }
        _ =>{
            if args[0]=="-r".to_string() {
                for arg in args.iter().skip(1) {
                    println!("{}:",arg);
                    let _ = do_ls(&arg,true);
                }
                Ok(())
            } else {
                for arg in args {
                    println!("{}:",arg);
                    let _ = do_ls(&arg,false);
                }
                Ok(())
            }
        }
    }
}

fn do_ls(dirname:&str,rec:bool)->Result<(),Box<dyn Error>>{
    let dirs = match rec {
        true => {
            let mut dirs:Vec<_> = fs::read_dir(dirname)?.filter_map(Result::ok).collect();
            let _ = dirs.sort_by(|a,b|{
                a.file_name().cmp(&b.file_name())
            });
            dirs
        },
        false => fs::read_dir(dirname)?.filter_map(Result::ok).collect()
    };
    for dir in dirs {
        println!("{}",dir.file_name().into_string().unwrap());
    }
    Ok(())
}