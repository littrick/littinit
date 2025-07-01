use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .header_contents("tty_wraper.h", "#include<sys/ioctl.h>")
        .allowlist_var("TIOCSCTTY")
        .generate()
        .expect("Unable to generate bindings");

    // 输出rs文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
