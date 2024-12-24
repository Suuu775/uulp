use std::{error::Error, fs::{create_dir_all, read_dir, OpenOptions}, io::{BufReader, BufWriter, Read, Write}, os::unix::fs::MetadataExt, path::{Path, PathBuf}, process::exit};
use clap::Parser;

const BUFFERSIZE: usize = 4096;

#[derive(Parser)]
#[command(name = "cp3")]
#[command(about = "Copy SOURCE to DEST", long_about = None)]
// 1. cp filea fileb  and filea != fileb = copy filea to fileb
// 2. cp filea fileb and filea == fileb = not copy
// 3. cp file dict = copy file to dict/file same with 1 2
// 4. cp -r dicta dictb = copy all dicta file to dictb
// 5. cp -r dict non-dict = cp: cannot overwrite non-directory 'non-dict' with directory 'dict/'
// 6. cp source _ and  source either not file or dict  = cp: cannot stat 'source': No such file or directory
// 7. cp dicta dictb = cp: -r not specified; omitting directory 'dicta'
struct Cli{
    #[arg(short)]
    rec: bool,
    #[arg(required = true)]
    source:String,
    #[arg(required = true)]
    dest:String
}

fn main()->Result<(),Box<dyn Error>>{
    let cli = Cli::parse();
    let source_path = Path::new(&cli.source);
    let dest_path = Path::new(&cli.dest);
    if source_path.is_file()&&dest_path.is_file() {
        if is_same_file(source_path, dest_path) { // 2
            eprintln!("cp: '{}' and '{}' are the same file",
            source_path.file_name().unwrap().to_str().unwrap(),
            dest_path.file_name().unwrap().to_str().unwrap());
            exit(1)
        } else { // 1
            return file2file(source_path, dest_path);
        }
    }

    if source_path.is_file()&&dest_path.is_dir() { // 3
        let source_filename = source_path.file_name().unwrap();
        let mut dest_path = PathBuf::from(dest_path);
        dest_path.push(source_filename);
        if is_same_file(source_path, &dest_path) {
            eprintln!("cp: '{}' and '{}' are the same file",
            source_filename.to_str().unwrap(),
            dest_path.file_name().unwrap().to_str().unwrap());
            exit(1)
        } else {
            return file2file(source_path, &dest_path);
        }
    }

    if !source_path.is_file() && !source_path.is_dir(){ //6
        eprintln!("cp: cannot stat '{}': No such file or directory",source_path.file_name().unwrap().to_str().unwrap());
        exit(1)
    }

    if source_path.is_dir()&&(dest_path.is_dir()||!dest_path.exists())&&cli.rec==true { // 4
        return copy_dirs_rec(source_path,dest_path);
    } else if cli.rec==false { // 7
        eprintln!("cp: -r not specified; omitting directory '{}'",source_path.file_name().unwrap().to_str().unwrap());
        exit(1)
    }

    if source_path.is_dir()&&!dest_path.is_dir()&&!dest_path.exists() { // 5
        eprintln!("cp: cannot overwrite non-directory '{}' with directory '{}'",source_path.file_name().unwrap().to_str().unwrap(),dest_path.file_name().unwrap().to_str().unwrap());
        exit(1)
    }

    Ok(())
}

fn is_same_file(source_file:&Path,dest_file:&Path)->bool{
    if !dest_file.exists() {
        return false;
    }

    let source_metadata = source_file.metadata().map_err(|e|{
        eprintln!("{}",e);
        exit(1)
    });

    let dest_metadata = dest_file.metadata().map_err(|e|{
        eprintln!("{}",e);
        exit(1)
    });

    match (source_metadata,dest_metadata) {
        (Ok(meta_src), Ok(meta_dest)) => {
            if meta_src.ino() == meta_dest.ino() && meta_src.dev() == meta_dest.dev() {
                return true;
            } else {
                return false;
            }
        }
        _ => unreachable!(),
    };
}

fn file2file(source_path:&Path,dest_path:&Path)->Result<(),Box<dyn Error>>{
    
    let source_file = OpenOptions::new().read(true).open(source_path);
    let dest_file = OpenOptions::new().write(true).create(true).open(dest_path);

    let source_file = source_file.map_err(|e|{
        eprintln!("{}",e);
        exit(1)
    });
    let dest_file = dest_file.map_err(|e|{
        eprintln!("{}",e);
        exit(1)
    });

    let (source_file,dest_file) = match (source_file,dest_file) {
        (Ok(s),Ok(d))=>(s,d),
        _ => unreachable!()
    };

    let (mut source_reader, mut dest_writer) = (BufReader::new(source_file), BufWriter::new(dest_file));
    
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

fn copy_dirs_rec(source_path: &Path, dest_path: &Path)->Result<(), Box<dyn Error>>{
    if !dest_path.exists() {
        create_dir_all(dest_path)?;
    }
    for entry in read_dir(source_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_entry_path = dest_path.join(entry.file_name());
        if entry_path.is_dir() {
            copy_dirs_rec(&entry_path, &dest_entry_path)?;
        } else {
            file2file(&entry_path, &dest_entry_path)?;
        }
    }
    Ok(())
}