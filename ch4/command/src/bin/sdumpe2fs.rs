// very very simply dumpe2fs
// refer to https://archive.kernel.org/oldwiki/ext4.wiki.kernel.org/index.php/Ext4_Disk_Layout.html
use std::{
    error::Error,
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom},
};
use clap::Parser;
#[derive(Parser)]
struct Cli {
    device: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf: [u8;4096] = [0; 4096];
    let device = Cli::parse().device;
    let mut file = OpenOptions::new().read(true).open(device)?;
    file.seek(SeekFrom::Start(1024))?;
    file.read_exact(&mut buf)?;
    let s_creator_os = match u32::from_le_bytes(buf[72..76].try_into()?) {
        0 => "Linux",
        1 => "Hurd",
        2 => "Masix",
        3 => "FreeBSD",
        4 => "Lites",
        _ => unimplemented!()
    };
    println!("Filesystem OS type: {}",s_creator_os);
    println!("Inode count: {}",u32::from_le_bytes(buf[0..4].try_into()?));
    println!("Block count: {}",u32::from_le_bytes(buf[4..8].try_into()?));
    let block_size = u32::from_le_bytes(buf[24..28].try_into()?);
    println!("Block size: {}",2_u32.pow(10+block_size));
    Ok(())
}
