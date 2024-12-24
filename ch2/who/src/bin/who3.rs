use libc::utmpx;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    ptr,
};
const UTMP_FILE: &str = "/var/run/utmp";
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("argument error");
        return Ok(());
    }
    let utmp_type = args[0].parse::<i16>()?;
    let utmp_size = size_of::<utmpx>();
    let utmp_file = File::open(UTMP_FILE)?;
    let mut reader = BufReader::new(utmp_file);
    let mut buf = vec![0u8; utmp_size];
    while let Ok(()) = reader.read_exact(&mut buf) {
        unsafe {
            let utmp_ptr: *const utmpx = buf.as_ptr() as *const utmpx;
            let utmp = ptr::read(utmp_ptr);
            show_info(utmp, utmp_type);
        }
    }
    Ok(())
}

fn show_info(utmp: utmpx, utmp_type: i16) {
    if utmp.ut_type == utmp_type {
        print!("{} ", utmp.ut_type);
        print!("{} ", utmp.ut_pid);
        print!("{} ", array_to_string(&utmp.ut_id));
        print!("{} {} ", utmp.ut_exit.e_termination, utmp.ut_exit.e_exit);
        print!("{:?}", utmp.ut_addr_v6);
        print!("{:<8} ", array_to_string(&utmp.ut_user));
        print!("{:<8} ", array_to_string(&utmp.ut_line));
        print!("{} {} ", utmp.ut_tv.tv_sec, utmp.ut_tv.tv_usec);
        println!("({})", array_to_string(&utmp.ut_host));
    }
}

fn array_to_string(arr: &[i8]) -> String {
    let bytes = unsafe { &*(&arr[..] as *const _ as *const [u8]) };
    let null_term_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let str_slice = unsafe { std::str::from_utf8_unchecked(&bytes[..null_term_pos]) };
    str_slice.to_string()
}
