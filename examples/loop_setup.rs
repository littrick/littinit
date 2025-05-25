use anyhow::{Ok, Result};
use littinit::util_linux::loopdev::set_loop;

fn main() -> Result<()> {
    let loopdev = set_loop("file")?;

    println!("loopdev: {loopdev:?}");

    Ok(())
}
