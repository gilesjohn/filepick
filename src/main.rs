#![cfg_attr(feature = "no_terminal", windows_subsystem = "windows")]

use std::{
    env::{self},
    fs,
    io::{self, Write},
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

fn pick_files(start_dir: PathBuf) -> Vec<PathBuf> {
    FileDialog::new()
        .set_directory(start_dir)
        .pick_files()
        .unwrap_or_default()
}

fn output_and_copy<W: Write>(
    file_paths: Vec<PathBuf>,
    copy_cwd: bool,
    overwrite: bool,
    separator: String,
    out: &mut W,
) -> io::Result<()> {
    let cwd = env::current_dir().unwrap();

    for file in file_paths {
        write!(out, "{}{}", file.display(), separator)?;

        if copy_cwd && let Some(file_name) = file.file_name() {
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

    Ok(())
}

const HELP_TEXT: &str = r#"filepick - Interactive file picker

Usage: filepick [start_path] [OPTIONS]

Arguments:
  [start_path]      Initial directory to open (default: current directory)

Options:
  --copy-to-cwd     Copy selected files to working directory
  --overwrite       Overwrite existing files when copying
  --null            Separate file paths with NULL character instead of newline
  -h, --help        Show this help message
"#;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{}", HELP_TEXT);
        return;
    }

    let options = parse_args(args);
    let file_paths = pick_files(options.start_dir);
    output_and_copy(
        file_paths,
        options.cwd_copy,
        options.overwrite,
        options.separator,
        &mut io::stdout(),
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        let args = vec![
            "filepick".to_string(),
            "--null".to_string(),
            "C:/Users".to_string(),
            "--copy-to-cwd".to_string(),
            "--overwrite".to_string(),
        ];
        let options = parse_args(args);
        assert_eq!(options.start_dir, PathBuf::from("C:/Users"));
        assert!(options.cwd_copy);
        assert!(options.overwrite);
        assert_eq!(options.separator, "\0");
    }

    #[test]
    fn test_output_and_copy() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        fs::create_dir(&source_dir).unwrap();

        let file1 = source_dir.join("file1.txt");
        let file2 = source_dir.join("file2.txt");
        fs::write(&file1, "Hello").unwrap();
        fs::write(&file2, "World").unwrap();

        let original_cwd = env::current_dir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        let mut output_buffer = Vec::new();
        output_and_copy(
            vec![file1.clone(), file2.clone()],
            true,
            true,
            "\n".to_string(),
            &mut output_buffer,
        )
        .unwrap();

        // Check output
        let output_string = String::from_utf8(output_buffer).unwrap();
        assert_eq!(
            output_string,
            format!("{}\n{}\n", file1.display(), file2.display())
        );

        // Check files were copied
        assert!(temp_dir.path().join("file1.txt").exists());
        assert!(temp_dir.path().join("file2.txt").exists());

        // Clean up
        env::set_current_dir(original_cwd).unwrap();
        temp_dir.close().unwrap();
    }
}
