use core::str;
use libc::utmpx;
use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    ptr,
};
const UTMP_FILE: &str = "/var/run/utmp";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0
        || (args.len() == 2 && args[0] == "am".to_string() && args[1] == "i".to_string())
    {
        let reclen = size_of::<utmpx>();
        if let Ok(file) = File::open(UTMP_FILE) {
            let mut reader = BufReader::new(file);
            let mut buf = vec![0u8; reclen];
            while let Ok(()) = reader.read_exact(&mut buf) {
                unsafe {
                    let utmp_ptr: *const utmpx = buf.as_ptr() as *const utmpx;
                    let utmp = ptr::read(utmp_ptr);
                    show_info(utmp);
                }
            }
        } else {
            eprintln!("can't open utmp file");
        }
    } else {
        eprintln!("argument error");
        return;
    }
}

fn show_info(utmp: utmpx) {
    if utmp.ut_type != 7 {
        return;
    }
    print!("{:<8} ", array_to_string(&utmp.ut_user));
    print!("{:<8} ", array_to_string(&utmp.ut_line));
    show_time(utmp.ut_tv.tv_sec as i64);
    #[cfg(SHOWHOST)]
    println!("({})", array_to_string(&utmp.ut_host));
}

fn array_to_string(arr: &[i8]) -> String {
    // &[i8] to &[u8]
    let bytes = unsafe { &*(&arr[..] as *const _ as *const [u8]) };
    String::from_utf8_lossy(bytes)
        .trim_end_matches('\0')
        .to_string()
}

fn show_time(time: i64) {
    let mut buffer = [0u8; 24];
    let time_ptr = &time as *const i64 as *const libc::time_t;
    let tm_info = unsafe { libc::localtime(time_ptr) };

    if tm_info.is_null() {
        eprintln!("localtime returned NULL");
        return;
    }

    unsafe {
        libc::strftime(
            buffer.as_mut_ptr() as *mut libc::c_char,
            size_of::<[i8; 24]>() as libc::size_t,
            b"    %Y-%m-%d %H:%M\0".as_ptr() as *const libc::c_char,
            tm_info,
        );
    }

    let time_str = str::from_utf8(buffer.as_slice())
        .unwrap_or_default()
        .trim_end_matches('\0');
    println!("{}", time_str);
}
