pub mod session_handler {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::env;

    pub fn create(pName: &str) {
        println!("Creating session for profile \"{}\"...", pName);
        println!("{}",read_aws_config_file());
    }

    pub fn show() {
        println!("Showing sessions...");
    }

    pub fn refresh() {
        println!("Refreshing sessions...");
    }

    pub fn clean() {
        println!("Cleaning sessions...");
    }

    fn read_aws_config_file() -> String {
        let mut path = match env::home_dir() {
            Some(path) => path,
            None => panic!("Could not retrieve user's home directory."),
        };
        let config_file_path = format!("{}/.aws/config", path.display());

        let f = File::open(config_file_path).expect("Could not find AWS config file.");
        let mut buf_reader = BufReader::new(f);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("Found config file but could not read it.");

        contents
    }
}
