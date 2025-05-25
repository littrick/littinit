use littinit::util_linux::switch_root::switch_root;
use std::env::args;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    switch_root(&args[1], "/bin/sh", vec![])
}