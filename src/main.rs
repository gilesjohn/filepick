#![cfg_attr(feature = "no_terminal", windows_subsystem = "windows")]

use std::{
    env::{self},
    fs,
    path::PathBuf,
};

use rfd::FileDialog;

struct PickOptions {
    start_dir: PathBuf,
    cwd_copy: bool,
    overwrite: bool,
    separator: String,
}

fn parse_args(args: Vec<String>) -> PickOptions {
    // First non-option argument will be the initial path to open in the picker, cwd if not provided
    let start_dir = args
        .iter()
        .filter(|p| !p.starts_with("--"))
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| ".".into());

    // If true, copy selected files to working directory
    let cwd_copy = if cfg!(feature = "default_copy") {
        true
    } else {
        args.iter().any(|a| a == "--copy-to-cwd")
    };

    // If true, overwrite existing files when copying
    let overwrite = if cfg!(feature = "always_copy") {
        true
    } else {
        args.iter().any(|a| a == "--overwrite")
    };

    // If true, separate file names with NULL character instead of line feed
    let null_sep = args.iter().any(|a| a == "--null");
    let sep = if null_sep { "\0" } else { "\n" };

    PickOptions {
        start_dir: start_dir.to_path_buf(),
        cwd_copy,
        overwrite,
        separator: sep.to_owned(),
    }
}

fn pick(options: PickOptions) {
    let file_paths = FileDialog::new()
        .set_directory(options.start_dir)
        .pick_files()
        .unwrap_or_default();

    let cwd = env::current_dir().unwrap();

    for file in file_paths {
        print!("{}{}", file.display(), options.separator);

        if options.cwd_copy
            && let Some(file_name) = file.file_name()
        {
            let dest = cwd.join(file_name);

            if !options.overwrite {
                let dest_exists = fs::exists(&dest);
                if dest_exists.is_err() || dest_exists.is_ok_and(|e| e) {
                    eprintln!("File already exists in destination: {}", dest.display());
                    continue;
                }
            }

            match fs::copy(&file, &dest) {
                Ok(_) => {}
                Err(_) => eprintln!(
                    "Failed to copy:\n\t{}\n\t{}",
                    file.display(),
                    dest.display(),
                ),
            }
        }
    }
}

fn main() {
    if env::args().any(|a| a == "help" || a == "--help" || a == "-h") {
        println!("Usage: filepick [start_path] [--copy-to-cwd] [--overwrite] [--null]");
        return;
    }

    let options = parse_args(env::args().collect());
    pick(options);
}
