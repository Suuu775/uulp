use core::str;
use std::{fs::File, io::{BufReader, Read}, ptr};
use libc::utmpx;

const UTMP_FILE:&str = "/var/run/utmp";

fn main(){
    let reclen = size_of::<utmpx>();
    if let Ok(file) = File::open(UTMP_FILE) {
        let mut reader = BufReader::new(file);
        let mut buf = vec![0u8;reclen];
        while let Ok(()) = reader.read_exact(&mut buf) {
            unsafe {
                let utmp_ptr: *const utmpx = buf.as_ptr() as *const utmpx;
                let utmp = ptr::read(utmp_ptr);
                show_info(utmp);
            }
        }
    }else {
        eprintln!("can't open utmp file");
    }
}

fn show_info(utmp:utmpx){
    print!("{:<8} ",array_to_string(&utmp.ut_user));
    print!("{:<8} ",array_to_string(&utmp.ut_line));
    print!("{} {} ",utmp.ut_tv.tv_sec,utmp.ut_tv.tv_usec);
    println!("({})",array_to_string(&utmp.ut_host));
}

fn array_to_string(arr:&[i8])->String{
    let bytes = unsafe { &*(&arr[..] as *const _  as *const [u8]) };
    let null_term_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let str_slice = unsafe { std::str::from_utf8_unchecked(&bytes[..null_term_pos]) };
    str_slice.to_string()
}