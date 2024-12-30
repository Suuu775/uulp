use std::{error::Error, fs::DirBuilder, path::Path};

fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        let path = Path::new(&arg);

        DirBuilder::new()
        .recursive(true)
        .create(path)?;
    }
    Ok(())
}