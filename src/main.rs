use std::env;
use std::path::{PathBuf};
use std::fs;
use std::io;

extern crate fs_extra;
use fs_extra::file;

fn load_args() -> Result<(PathBuf, PathBuf), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err(String::from("src dir = ?, dst dir = ?"))
    }
    else if args.len() < 3 {
        Err(String::from(format!("src dir = {}, dst dir = ?", args[1])))
    }
    else {
        let src = PathBuf::from(&args[1]);
        let dst = PathBuf::from(&args[2]);

        Ok((src, dst))
    }
}

fn load_xbooks(dir: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut files: Vec<PathBuf> = Vec::new();

    for p in fs::read_dir(dir)? {
        let path = p?.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                match ext.to_os_string().into_string() {
                    Ok(s) => if s == "zip" { files.push(path.to_path_buf()); }
                    Err(e) => println!("{}", e.into_string().unwrap()),
                }
            }
        }
    }
    Ok(files)
}

fn move_file(src: &PathBuf, dst: &PathBuf) -> Result<u64, fs_extra::error::Error>{
    let opt = file::CopyOptions::new();
    Ok(file::move_file(src, dst, &opt)?)
}

fn main() -> Result<(), String>{
    let (src, dst) = load_args()?;

    for src_file in load_xbooks(&src).unwrap() {
        let mut dst_file = PathBuf::new();
        dst_file.push(&dst);
        dst_file.push(src_file.file_name().unwrap());
        match move_file(&src_file, &dst_file) {
            Ok(_) => println!("{} -> {}", src_file.display(), dst_file.display()),
            Err(e) => println!("{}", e.to_string()),
        }
    }

    Ok(())
}
