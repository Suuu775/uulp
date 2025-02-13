use std::env::args;

fn main(){
    let argv = args();
    println!("Number of args :{},Args are:",argv.len());
    for (i,arg) in argv.enumerate() {
        println!("args[{}] {}",i,arg);
    }
    eprint!("This message is sent to stderr");
}