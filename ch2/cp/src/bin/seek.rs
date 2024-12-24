use std::{fs::OpenOptions, io::{Read, Seek, SeekFrom, Write}, process::exit};

fn main(){
    let mut new_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create_new(true)
    .open("foo.txt")
    .map_err(|err|{
        eprintln!("{}",err);
        exit(1);
    })
    .unwrap();
    let mut buf = vec!['a' as u8;64];
    let _ = new_file.write_all(&mut buf);
    let _ = new_file.seek(SeekFrom::End(20000));
    let mut buf1 = vec!['a' as u8;16];
    let _ = new_file.read(&mut buf1);
    // println!("{:?}",buf1);
    let _ = new_file.write_all("hello".as_bytes());
    // let exist_file = File::open("foo.txt").unwrap();
    // println!();
    // for b in exist_file.bytes(){
    //     print!("{}",b.unwrap());
    // }
}