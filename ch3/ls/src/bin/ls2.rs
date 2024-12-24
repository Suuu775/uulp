use std::{env, error::Error, fs::{self, metadata, Metadata}, os::linux::fs::MetadataExt, path::Path};
use nix::{libc::{self}, unistd::{Gid, Group}};
use nix::unistd::{Uid, User};
fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 =>{
            do_ls(".")
        },
        _ =>{
            for filename in args {
                println!("{}:",filename);
                let _ = do_ls(&filename);
            }
            Ok(())
        }
    }
}

fn do_ls(filename:&str)->Result<(),Box<dyn Error>>{
    let filetype = metadata(filename)?.file_type();
    if filetype.is_dir() {
        let mut read_dir:Vec<_> =  fs::read_dir(filename)?.filter_map(Result::ok).collect();
        read_dir.sort_by_key(|entry|entry.file_name());
        for dir_entry in read_dir  {
            let _ = dostat(dir_entry.path().to_str().unwrap()); // fix cargo run --bin ls2 /tmp/
        }
    } else {
        let _ = dostat(filename);
    }
    Ok(())
}

fn dostat(filename:&str)->Result<(),Box<dyn Error>>{
    let stat = metadata(&filename)?;
    show_file_info(stat,&filename);
    Ok(())
}

fn show_file_info(stat:Metadata,filename:&str){
    print!("{} ",mode_to_string(std::os::unix::fs::MetadataExt::mode(&stat)));
    print!("{} ",std::os::unix::fs::MetadataExt::nlink(&stat));
    print!("{} ",uid_to_name(std::os::unix::fs::MetadataExt::uid(&stat)));
    print!("{} ",gid_to_name(std::os::unix::fs::MetadataExt::gid(&stat)));
    print!("{} ",stat.st_size());
    print!("{} ",show_time(std::os::unix::fs::MetadataExt::atime(&stat)));
    let filename = Path::new(filename).file_name().unwrap().to_str().unwrap();// fix cargo run --bin ls2 /tmp/
    println!("{}",filename);
}

fn mode_to_string(mode: u32) -> String {
    let mut str = "----------".to_string();

    if mode & libc::S_IFDIR != 0 {
        str.replace_range(0..1, "d");
    } else if mode & libc::S_IFCHR != 0 {
        str.replace_range(0..1, "c");
    } else if mode & libc::S_IFBLK != 0 {
        str.replace_range(0..1, "b");
    }

    if mode & libc::S_IRUSR != 0 {
        str.replace_range(1..2, "r");
    }
    if mode & libc::S_IWUSR != 0 {
        str.replace_range(2..3, "w");
    }
    if mode & libc::S_ISUID != 0 {
        str.replace_range(3..4, "s");
    } else if mode & libc::S_IXUSR != 0 {
        str.replace_range(3..4, "x");
    }

    if mode & libc::S_IRGRP != 0 {
        str.replace_range(4..5, "r");
    }
    if mode & libc::S_IWGRP != 0 {
        str.replace_range(5..6, "w");
    }
    if mode & libc::S_ISGID != 0 {
        str.replace_range(6..7, "s");
    } else if mode & libc::S_IXGRP != 0 {
        str.replace_range(6..7, "x");
    }

    if mode & libc::S_IROTH != 0 {
        str.replace_range(7..8, "r");
    }
    if mode & libc::S_IWOTH != 0 {
        str.replace_range(8..9, "w");
    }
    if mode & libc::S_ISVTX != 0 {
        str.replace_range(9..10, "t");
    } else if mode & libc::S_IXOTH!=0 {
        str.replace_range(9..10, "x");
    }

    str
}

fn uid_to_name(uid:u32)->String{
    return User::from_uid(Uid::from_raw(uid)).unwrap().unwrap().name;
}

fn gid_to_name(gid:u32)->String{
    Group::from_gid(Gid::from_raw(gid)).unwrap().unwrap().name
}

fn show_time(seconds: i64) -> String {
    let naive = chrono::DateTime::from_timestamp(seconds, 0).unwrap();
    let time = naive.format("%Y-%m-%d %H:%M:%S").to_string();
    time
}