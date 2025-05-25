use std::env;
use std::path::PathBuf;

fn main() {
    // println!("cargo:rustc-link-search=/path/to/lib");
    // println!("cargo:rustc-link-lib=bz2");

    let allow_items = [
        "LOOPIO_.*", 
        "LOOPCTL_.*",
        "loop_info.*"
        ].join("|");

    // 查找.h文件
    let h_files: Vec<String> = PathBuf::from("bindgen")
        .read_dir()
        .unwrap()
        .filter_map(|r| match r {
            Ok(e) if e.path().is_file() && e.file_name().into_string().unwrap().ends_with(".h") => {
                Some(e.path().to_str().unwrap().to_string())
            }
            _ => None,
        })
        .collect();

    // 生成bind
    let bindings = bindgen::Builder::default()
        .headers(h_files)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_item(allow_items)
        .generate()
        .expect("Unable to generate bindings");

    // 输出rs文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
