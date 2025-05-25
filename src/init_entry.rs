use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub enum InitEntry {
    Change {
        uppper: PathBuf,
        lowwer: Vec<PathBuf>,
        init: PathBuf,
        args: Vec<String>,
        env: HashMap<String, String>,
    },
    Switch {
        root_dev: PathBuf,
        init: PathBuf,
        args: Vec<String>,
        env: HashMap<String, String>,
    },
    Kernel {
        kernel: PathBuf,
        initrd: PathBuf,
        args: Vec<String>,
    },
}


impl InitEntry {
    pub fn run(&self){
        
    }
}