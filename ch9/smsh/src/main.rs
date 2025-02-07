use execute::execute::{execute, next_cmd};
use libc::{signal, SIGINT, SIGQUIT, SIG_IGN};
use splitline::splitline::splitline;

mod execute;
mod splitline;

const DEL_PROMPT:&str = "> ";

fn main() {
    setup();
    while let Some(cmdline) = next_cmd(DEL_PROMPT) {
        let arglist = splitline(cmdline);
        execute(arglist);
    }
}

fn setup(){
    unsafe {signal(SIGINT, SIG_IGN)};
    unsafe {signal(SIGQUIT, SIG_IGN)};
}