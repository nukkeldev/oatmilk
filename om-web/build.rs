use std::{fs, path::Path, process::Command};

fn main() {
    remove_dir_if_exists("build/");
    copy_dir("assets/scripts/", "build/scripts/");
    copy_dir("assets/vector/", "build/vector/");
    copy_dir("assets/images/", "build/images/");
    copy_dir("assets/fonts/", "build/fonts/");
    build_sass_files();
}

fn build_sass_files() {
    _ = Command::new("sass")
        .arg("assets/styles/:build/styles/")
        .status()
        .expect("failed to execute sass");
}

fn remove_dir_if_exists(dir: &str) {
    let path = Path::new(dir);
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

fn copy_dir(src: &str, dst: &str) {
    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    if !dst_path.exists() {
        fs::create_dir_all(dst_path).unwrap();
    }

    println!("cargo:rerun-if-changed={dst}");

    for entry in fs::read_dir(src_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = entry.file_name();
        let dst_file = dst_path.join(file_name);

        if path.is_file() {
            fs::copy(&path, &dst_file).unwrap();
        } else if path.is_dir() {
            copy_dir(path.to_str().unwrap(), dst_file.to_str().unwrap());
        }
    }
}
