use std::fs;
use std::process::exit;
use crate::logger;
use crate::conf::configuration_dto::Configuration;

impl Configuration {
    pub fn init_configs(config_dir: &str) -> Configuration {
        let config = Configuration::from_file(&format!("{config_dir}/core-x.toml"));
        config
    }


    pub fn init_config_dir(config_dir: &str) {
        let mut log = logger::get_logger();

        log.info(&format!("Creating configuration directory: {config_dir}"));

        // Check Config Directory exists
        if std::path::Path::new(config_dir).exists() {
            log.info(&format!("Directory already exists: {config_dir}"));
        } else {
            fs::create_dir(config_dir).unwrap();
        }
    }


    fn init_to_file(config_path: &str) {
        let mut log = logger::get_logger();

        let basic_config_path = format!("{config_path}/core-x.toml");


        log.info(&format!("Writing Default configuration file: {basic_config_path}"));

        // Check file exists
        if std::path::Path::new(config_path).exists() {
            log.info(&format!("File already exists: {config_path}"));
            return;
        }

        let config = Configuration::default();
        let d = toml::to_string(&config).unwrap();
        fs::write(config_path, d).unwrap();
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
            Err(er) => {
                // Write `msg` to `stderr`.
                log.error(&format!("Unable to load data from `{file_path} {er}`"));
                // Exit the program with exit code `1`.
                exit(1);
            }
        }
    }
}