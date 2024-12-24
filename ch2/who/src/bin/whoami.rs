use std::{fs::File, io::Read,ptr};

use libc::utmpx;
const UTMP_FILE:&str = "/var/run/utmp";
fn main()->Result<(),std::io::Error>{
    let utmp_size = size_of::<utmpx>();
    let mut file = File::open(UTMP_FILE)?;
    let mut buf = vec![0u8;utmp_size];
    while let Ok(()) = file.read_exact(&mut buf) {
        unsafe {
            let utmp_ptr = buf.as_ptr() as *const utmpx;
            let utmp = ptr::read(utmp_ptr);
            if utmp.ut_type!=7 {
                continue;
            } else {
                println!("{}",array_to_string(&utmp.ut_user));
            }
        }
    }
    Ok(())
}

fn array_to_string(arr:&[i8])->String{
    // &[i8] to &[u8]
    let bytes = unsafe { &*(&arr[..] as *const _  as *const [u8]) };
    String::from_utf8_lossy(bytes).trim_end_matches('\0').to_string()
}