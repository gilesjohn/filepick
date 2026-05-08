use std::{env, path::PathBuf};

use rfd::FileDialog;

// ## ADDING COMMITS RETROACTIVELY AS I DID IT ALL IN ONE GO, THESE ARE MUCH CLEANER THAN MY NORMAL INITIAL COMMITS 😅

fn main() {
    let start_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| ".".into());

    let file_paths = FileDialog::new()
        .set_directory(start_dir)
        .pick_files()
        .unwrap_or_default();

    for file in file_paths {
        println!("{}", file.display());
    }
}
