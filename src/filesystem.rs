use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs;

extern crate walkdir;
use self::walkdir::WalkDir;

use errors::RustfluxError;
use chrono::prelude::*;

/// Saves the contents of `data` to the file `dir/name`. Returns Ok(file_name) representing
/// the full path of the saved file on success, `Err(Rustflux::IOError(reason))` otherwise
pub fn save_file_to_disk(dir: &str, name: &str, data: &[String]) -> Result<String, RustfluxError> {
    match fs::create_dir_all(dir) {
        Ok(_) => {}
        Err(_) => {
            return Err(RustfluxError::IOError(String::from(
                "Cannot create /tmp/.rustflux",
            )))
        }
    };

    let utc = Utc::now().timestamp();
    let file_name = format!("{}/{}_{}", dir, name, utc);
    {
        let path = Path::new(&file_name);

        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(RustfluxError::IOError(format!(
                    "Cannot create file for measurement: {}",
                    err
                )));
            }
        };

        for line in data.iter() {
            let _ = file.write(line.as_bytes()).unwrap();
            let _ = file.write(b"\n").unwrap();
        }
    }
    Ok(file_name)
}

/// Walks the directory `dir` and gets the file names. Returns `Ok(file_names_vec)` on success,
/// `Err(RustfluxError::IOError(reason))` otherwise
pub fn files_in_dir(dir: &str) -> Result<Vec<String>, RustfluxError> {
    let mut files: Vec<String> = Vec::new();

    let walker = WalkDir::new(dir).into_iter();

    // skip the folder itself
    for entry in walker.skip(1) {
        let entry = entry.unwrap();
        let file_name = format!("{}", entry.path().display());
        files.push(file_name);
    }

    Ok(files)
}

// TESTS

#[test]
fn test_extract_files_in_dir() {
    let dir = "tests/test_extract_name_folder";
    let files = files_in_dir(dir);

    assert_eq!(
        vec![
            "tests/test_extract_name_folder/iv.txt",
            "tests/test_extract_name_folder/vim.txt",
            "tests/test_extract_name_folder/elixir.txt",
        ],
        files.unwrap()
    );
}
