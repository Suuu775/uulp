use std::{
    env,
    error::Error,
    fs::{metadata, File},
    io::{stdin, BufReader, Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
    process,
};

const BUFFERSIZE: usize = 4096;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 2 && Path::new(&args[0]).exists() {
        return cp_judge(false, &args[0], &args[1]);
    } else if args.len() == 3 && args[0] == "-i".to_string() && Path::new(&args[1]).exists() {
        return cp_judge(true, &args[1], &args[2]);
    } else {
        eprintln!("argument error");
        process::exit(1)
    }
}

fn cp_judge(info: bool, source_path: &String, dest_path: &String) -> Result<(), Box<dyn Error>> {
    if info == true && Path::new(dest_path).exists() {
        eprint!("cp: overwrite '{}' ?", dest_path);
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        if line == "Y\n".to_string() {
            return cp(source_path, dest_path);
        } else {
            process::exit(1)
        }
    }
    return cp(source_path, dest_path);
}

fn cp(source_path: &String, dest_path: &String) -> Result<(), Box<dyn Error>> {
    if is_same_file(source_path, dest_path) == true {
        eprintln!(
            "cp: '{}' and '{}' are the same file",
            source_path, dest_path
        );
        return Ok(());
    }
    // open source file
    let source_file = File::open(source_path).map_err(|e| {
        eprintln!("can't open file {} : {}", source_path, e);
        std::process::exit(1);
    });

    // create or replace dest file
    let dest_file = File::create(dest_path).map_err(|e| {
        eprintln!("can't create file {} : {}", dest_path, e);
        std::process::exit(1);
    });

    // unwarp file from result
    let (source_file, dest_file) = match (source_file, dest_file) {
        (Ok(s), Ok(d)) => (s, d),
        _ => unreachable!(),
    };

    // create bufreader of source_reader and des_writer
    let (mut source_reader, mut dest_writer) = (BufReader::new(source_file), dest_file);

    // write source file to dest file
    let mut buf = vec![0u8; BUFFERSIZE];
    while let Ok(size) = source_reader.read(&mut buf) {
        if size == 0 {
            break;
        } else {
            let _ = dest_writer.write_all(&buf[..size]);
        }
    }
    Ok(())
}

fn is_same_file(source_path: &String, dest_path: &String) -> bool {
    // judge two file is same file
    if Path::new(source_path).exists() == false || Path::new(dest_path).exists() == false {
        return false;
    }

    let source_metadata = metadata(source_path).map_err(|err| {
        eprintln!("can't get metadata for {} : {}", source_path, err);
        std::process::exit(1);
    });

    let dest_metadata = metadata(dest_path).map_err(|err| {
        eprintln!("can't get metadata for {} : {}", dest_path, err);
        std::process::exit(1);
    });

    match (source_metadata, dest_metadata) {
        (Ok(meta_src), Ok(meta_dest)) => {
            if meta_src.ino() == meta_dest.ino() && meta_src.dev() == meta_dest.dev() {
                return true;
            } else {
                return false;
            }
        }
        _ => unreachable!(),
    }
}
