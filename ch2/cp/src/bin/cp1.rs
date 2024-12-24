use std::env;
use std::fs::{metadata, File};
use std::io::{BufReader, Read, Write};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

const BUFFERSIZE: usize = 4096;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        eprintln!("argument error");
        return;
    }

    let source_path = &args[0];
    let dest_path = &args[1];

    // judge two file is same or not
    if is_same_file(source_path, dest_path) == true {
        eprintln!(
            "cp: '{}' and '{}' are the same file",
            source_path, dest_path
        );
        return;
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
