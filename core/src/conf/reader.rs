use std::fs;
use std::process::exit;
use crate::logger;
use crate::conf::configuration_dto::Configuration;

impl Configuration {
    pub fn init_to_file(file_path: &str) {
        let mut log = logger::get_logger();
        log.info(&format!("Writing Default configuation file: {}", file_path));

        // Check file exists
        if std::path::Path::new(file_path).exists() {
            log.info(&format!("File already exists: {}", file_path));
            return;
        }

        let config = Configuration::default();
        let d = toml::to_string(&config).unwrap();
        std::fs::write(file_path, d).unwrap();
    }


    pub fn from_file(file_path: &str) -> Configuration {
        let mut log = logger::get_logger();
        log.info(&format!("Reading configuration from file: {}", file_path));

        // Read the contents of the file using a `match` block
        // to return the `data: Ok(c)` as a `String`
        // or handle any `errors: Err(_)`.
        let contents = match fs::read_to_string(file_path) {
            // If successful return the files text as `contents`.
            // `c` is a local variable.
            Ok(c) => c,
            // Handle the `error` case.
            Err(_) => {
                // Write `msg` to `stderr`.
                log.error(&format!("Unable to read file `{}`", file_path));
                // Exit the program with exit code `1`.
                exit(1);
            }
        };

        // Use a `match` block to return the
        // file `contents` as a `Data struct: Ok(d)`
        // or handle any `errors: Err(_)`.
        match toml::from_str(&contents) {
            // If successful, return data as `Data` struct.
            // `d` is a local variable.
            Ok(d) => d,
            // Handle the `error` case.
            Err(_) => {
                // Write `msg` to `stderr`.
                log.error(&format!("Unable to load data from `{}`", file_path));
                // Exit the program with exit code `1`.
                exit(1);
            }
        }
    }
}