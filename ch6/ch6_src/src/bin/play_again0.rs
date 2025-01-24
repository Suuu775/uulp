use std::{error::Error, io::{self, Read}, process::exit};

const QUESTION:&str = "Do you want another translation";

fn main(){
    get_response(QUESTION);
}

fn getchar()->Result<char,Box<dyn Error>>{
    let mut buffer = [0;1];
    let mut stdin = io::stdin().lock();
    stdin.read_exact(&mut buffer)?;
    let char = buffer[0] as char;
    Ok(char)
}

fn get_response(question:&str){
    println!("{} (y/n)?",question);
    loop {
        if let Ok(input) = getchar() {
            match input {
                'y'|'Y' =>{
                    exit(0)
                },
                'n'|'N'=>{
                    exit(1)
                },
                _=>{
                    continue;
                }
            }
        }
    }
}