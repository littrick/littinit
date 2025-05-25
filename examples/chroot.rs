use littinit::util_linux::chroot::chroot;
use std::{collections::HashMap, env::args};
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    chroot(&args[1], "/bin/sh", vec![], HashMap::new())
}