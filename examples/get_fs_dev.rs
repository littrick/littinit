use anyhow::Ok;
use anyhow::Result;

use nix::sys::statfs::fstatfs;
use nix::sys::stat::stat;
use std::env::args;
use std::{fs::File, os::unix::fs::MetadataExt, path::Path};

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    let path = Path::new(&args[1]);

    println!("path: {path:?}");

    println!("metadata.dev: {:?}", path.metadata()?.dev());
    println!("fstatfs: {:?}", fstatfs(File::open(path)?)?.filesystem_type());
    println!("stat: {:?}", stat(path.as_os_str())?.st_dev);

    Ok(())
}
