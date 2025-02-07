pub mod execute {
    use std::{
        io::{self, Write},
        process::Command,
    };

    use libc::{signal, SIGINT, SIGQUIT, SIG_DFL};
    pub fn execute(argv: Vec<String>) {
        unsafe { signal(SIGINT, SIG_DFL) };
        unsafe { signal(SIGQUIT, SIG_DFL) };
        if argv.len() != 0 {
            let output = Command::new(&argv[0])
                .args(&argv[1..])
                .output()
                .expect("Failed to execute command");

            let output =
                String::from_utf8(output.stdout).expect("Failed to printf execute command result");
            println!("{}", output);
        }
    }

    pub fn next_cmd(prompt: &str) -> Option<String> {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Some(input),
            Err(_) => None,
        }
    }
}
