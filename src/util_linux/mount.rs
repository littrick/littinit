use anyhow::{Ok, Result};
use std::path::Path;
use std::fmt::Debug;
use nix::mount::mount as nix_mount;
use nix::mount::umount as nix_umount;


pub use nix::mount::{MsFlags, MntFlags};


pub fn mount<P1, P2>(source: P1, target: P2, flags: MsFlags) -> Result<()>
where
    P1: AsRef<Path> + Debug,
    P2: AsRef<Path> + Debug,
{

    Ok(())
}
