use std::env;
use std::path::{PathBuf};
use std::fs;
use std::io;
use std::io::Write;

extern crate fs_extra;
use fs_extra::file;

extern crate regex;
use regex::Regex;

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

fn deploy_xbooks(src_files: &Vec<PathBuf>, dst_dir: &PathBuf) {
    let xbook_re: Regex = Regex::new(r"^\[(.+)\] .+\.zip$").unwrap();
    for src_file in src_files {
        //println!("{}", src_file.display());
        let fname = src_file.file_name().unwrap().to_string_lossy().into_owned();
        match xbook_re.captures(fname.as_str()) {
            Some(caps) => {
                let mut dst_auther_dir = PathBuf::from(dst_dir);
                dst_auther_dir.push(caps.get(1).map_or("", |m| m.as_str()));

                let mut dst_file = PathBuf::from(&dst_auther_dir);
                dst_file.push(src_file.file_name().unwrap());

                if ! dst_auther_dir.is_dir() {
                    match fs::create_dir(&dst_auther_dir) {
                        Ok(_) => println!("created dir {}", dst_auther_dir.display()),
                        Err(e) => println!("{}", e.to_string()),
                    }
                }

                let opt = file::CopyOptions::new();
                match file::move_file(&src_file, &dst_file, &opt) {
                    Ok(_) => println!("{} -> {}", fname, dst_file.display()),
                    Err(err) => println!("{}", err.to_string()),
                }
            },
            None => println!("invalid file name format - {}", fname),
        }
    }
}

fn move_file(src: &PathBuf, dst_dir: &PathBuf) {
    let mut dst = PathBuf::new();
    dst.push(&dst_dir);
    dst.push(src.file_name().unwrap());

    let opt = file::CopyOptions::new();
    match file::move_file(&src, &dst, &opt) {
        Ok(_) => println!("{} -> {}", src.display(), dst.display()),
        Err(e) => println!("{}", e.to_string()),
    }
}

fn press_any_key() -> Result<(), String> {
    print!("Press any key? ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    Ok(())
}

fn main() -> Result<(), String>{
    let (src_dir, dst_dir) = load_args()?;

    match load_xbooks(&src_dir) {
        Ok(src_files) => deploy_xbooks(&src_files, &dst_dir),
        Err(err) => println!("{}", err.to_string()),
    }
    /*
    for src_file in load_xbooks(&src_dir).unwrap() {
        move_file(&src_file, &dst_dir);
    }
    */

    //press_any_key()
    Ok(())
}
