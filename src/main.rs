use std::env;
use std::path::{Path, PathBuf};
use std::fs;

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

fn load_xbooks(dir: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut files: Vec<PathBuf> = Vec::new();

    match fs::read_dir(dir) {
        Ok
    for p in fs::read_dir(dir).unwrap() {
        let path = p.unwrap().path();
        if path.is_file() {
            match path.extension() {
                Some(ext) => {
                    match ext.to_os_string().into_string() {
                        Ok(s) => {
                            if s == "zip" {
                                files.push(path.to_path_buf());
                            }
                        },
                        Err(_) => (),
                    }
                },
                None => (),
            }
        }
    }

    Ok(files)
}

fn main() {
    let (src, dst) = match load_args() {
        Ok((s, d)) => (s, d),
        Err(e) => { 
            println!("{}", e);
            return
        },
    };
    println!("{} -> {}", src.display().to_string(), dst.display().to_string());

    for path in load_xbooks(&src).unwrap() {
        println!("{}", path.into_os_string().into_string().unwrap());
    }

    ()
}
