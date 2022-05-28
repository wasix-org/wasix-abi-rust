use std::env;
use std::path::PathBuf;

fn main() {
    let witx_paths = match env::args_os().nth(1) {
        Some(path) => vec![PathBuf::from(path)],
        None => witx::phases::snapshot().unwrap(),
    };
    let is64bit = match env::args().nth(2) {
        Some(path) if path == "64bit" => true,
        _ => false,
    };
    print!("{}", witx_bindgen::generate(&witx_paths, is64bit));
}
