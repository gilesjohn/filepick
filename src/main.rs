use std::{env, fs, path::PathBuf};

use rfd::FileDialog;

fn main() {
    // If true, copy selected files to working directory
    let cwd_copy = env::args().any(|a| a == "--copy-to-cwd");

    // If true, overwrite existing files when copying
    let overwrite = env::args().any(|a| a == "--overwrite");

    // First non-option argument will be the initial path to open in the picker
    let start_dir = env::args()
        .filter(|p| !p.starts_with("--"))
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| ".".into());

    let file_paths = FileDialog::new()
        .set_directory(start_dir)
        .pick_files()
        .unwrap_or_default();

    let cwd = env::current_dir().unwrap();

    for file in file_paths {
        println!("{}", file.display());

        if cwd_copy && let Some(file_name) = file.file_name() {
            let dest = cwd.join(file_name);

            if !overwrite {
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
