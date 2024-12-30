use std::{
    env::set_current_dir,
    error::Error,
    fs,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::exit,
};

fn main() -> Result<(), Box<dyn Error>> {
    let ino = get_inode(Path::new("."))?;
    printpathto(ino)?;
    Ok(())
}

fn printpathto(ino: u64) -> Result<(), Box<dyn Error>> {
    let pino = get_inode(Path::new(".."))?;
    if pino != ino {
        set_current_dir("..")?;
        let its_name = ino_to_name(ino)?;
        let my_inode = get_inode(Path::new("."))?;
        printpathto(my_inode)?;

        if let Some(its_name) = its_name.file_name() {
            print!("/{}", its_name.to_string_lossy());
        }
    }
    Ok(())
}

fn ino_to_name(ino: u64) -> Result<PathBuf, Box<dyn Error>> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let entry_meta = entry.metadata()?;
        if entry_meta.ino() == ino {
            return Ok(entry.path());
        }
    }
    eprintln!("error looking for {}\n", ino);
    exit(1);
}

fn get_inode(fname: &Path) -> Result<u64, Box<dyn Error>> {
    let fmeta = fs::metadata(fname)?;
    return Ok(fmeta.ino());
}
