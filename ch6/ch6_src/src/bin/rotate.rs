use std::{error::Error, io::{self, Read}};

fn main(){
    while let Ok(c) = getchar() {
        if c=='z' {
            print!("a");
        } else if c.is_lowercase() {
            print!("{}",(c as u8 +1) as char);
        }else {
            print!("{}",c);
        }
    }
}

fn getchar()->Result<char,Box<dyn Error>>{
    let mut buffer = [0;1];
    let mut stdin = io::stdin().lock();
    stdin.read_exact(&mut buffer)?;
    let char = buffer[0] as char;
    Ok(char)
}