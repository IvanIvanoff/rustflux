use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use errors::RustfluxError;
use chrono::prelude::*;

pub fn save_file_to_disk(
    dir: &str,
    measurement_name: &str,
    line_protocol: &[String],
) -> Result<String, RustfluxError> {
    match fs::create_dir_all(dir) {
        Ok(_) => {}
        Err(_) => {
            return Err(RustfluxError::IOError(String::from(
                "Cannot create /tmp/.rustflux",
            )))
        }
    };

    let utc = Utc::now().timestamp();
    let file_name = format!("{}/{}_{}", dir, measurement_name, utc);
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

        for line in line_protocol.iter() {
            let _ = file.write(line.as_bytes()).unwrap();
            let _ = file.write(b"\n").unwrap();
        }
    }
    Ok(file_name)
}
