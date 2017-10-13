extern crate regex;

extern crate rusoto_core;
extern crate rusoto_sts;
extern crate rusoto_dynamodb;

pub mod SessionHandler {
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, Error};
    use std::io;
    use std::io::prelude::*;
    use std::io::{stdin, stdout};
    use std::env;
    use rusoto_core::{DefaultCredentialsProvider, ProfileProvider,
        ProvideAwsCredentials, Region, default_tls_client};
    use rusoto_sts::{Sts, StsClient, GetSessionTokenRequest, GetSessionTokenResponse, GetSessionTokenError, Credentials};
    use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
    use regex::Regex;
    use std::collections::HashMap;

    pub struct SessionHandler {
        AWSConfig: AWSConfig,
    }

    struct AWSConfig {
        profiles: HashMap<String, AWSProfile>,
    }

    #[derive(Debug)]
    struct AWSProfile {
        source_profile: Option<String>,
        region: Option<String>,
        output: Option<String>,
        role_arn: Option<String>,
        mfa_serial: Option<String>,
        ot_session_name: Option<String>,
        ot_expiration: Option<String>,
        ot_source_profile: Option<String>,
        ot_role_arn: Option<String>,
    }
    
    impl AWSConfig {
        fn get_profile(&self, name: &str) -> &AWSProfile {
            &self.profiles[name]
        }
    }

    impl SessionHandler {
        pub fn load_config(&mut self) {
            let aws_config_file = read_aws_config_file();
            let profiles = split_config_file(aws_config_file);

            // Create an instance of the AWSConfig struct
            let mut aws_config = AWSConfig {
                profiles: HashMap::new()
            };

            // Iterate over profile chunks
            for profile in profiles {
                // Create a vector of string that holds each line
                let split = profile.split("\n");
                let lines: Vec<String> = split.map(|s| s.to_string()).collect();

                // Get the profile name from the first line
                let profile_line = lines[0].to_owned();

                // Lets ignore the default for now
                if profile_line == "[default]" {
                    continue;
                }

                let split2 = profile_line.split(" ");
                let mut words: Vec<String> = split2.map(|s| s.to_string()).collect();
                let mut profile_name = String::new();
                if words.len() > 1 {
                    words[1].pop();
                    profile_name = words[1].trim_right().to_owned();
                }

                let mut aws_profile = AWSProfile{
                    source_profile: None,
                    region: None,
                    output: None,
                    role_arn: None,
                    mfa_serial: None,
                    ot_expiration: None,
                    ot_session_name: None,
                    ot_source_profile: None,
                    ot_role_arn: None,
                };

                for i in 1..lines.len() {
                    let split = lines[i].split(" = ");
                    let config: Vec<String> = split.map(|s| s.to_string()).collect();
                    let key = &config[0];
                    let value = config[1].to_owned();
                    match key.as_ref() {
                        "mfa_serial" => aws_profile.mfa_serial = Some(value),
                        "output" => aws_profile.output = Some(value),
                        "region" => aws_profile.region = Some(value),
                        "role_arn" => aws_profile.role_arn = Some(value),
                        "source_profile" => aws_profile.source_profile = Some(value),
                        "ot_session_name" => aws_profile.ot_session_name = Some(value),
                        "ot_expiration" => aws_profile.ot_expiration = Some(value),
                        "ot_source_profile" => aws_profile.ot_source_profile = Some(value),
                        "ot_role_arn" => aws_profile.ot_role_arn = Some(value),
                        _ => (),
                    }
                }

                if profile_name != "" {
                    aws_config.profiles.insert(profile_name.clone(), aws_profile);
                }
            }

            self.AWSConfig = aws_config;
        }

        pub fn create(&self, profile_name: &str) {
            println!("Creating session for profile \"{}\"...", profile_name);
            //let aws_profile = &self.AWSConfig.profiles[profile_name];
            let aws_profile = &self.AWSConfig.get_profile(profile_name);

            let mut provider = ProfileProvider::new().unwrap();
            provider.set_profile(profile_name);
            // let region = &aws_profile.region;
            // match region {
            //     &Some(ref region) => region,
            //     &None => panic!("You must specify a region for profile {}", profile_name)
            // };
            // println!("Region: {:?}", region.as_ref().unwrap());
            let client = StsClient::new(default_tls_client().unwrap(), provider, Region::EuWest1);

            let mfa_serial = &aws_profile.mfa_serial;
            if mfa_serial.is_some() {
                print!("Enter AWS MFA code for profile [{}]: ", profile_name);
                stdout().flush();
                let mut token_code = String::new();
                stdin().read_line(&mut token_code)
                    .ok()
                    .expect("Couldn't read line");    
                token_code.pop(); // Remove newline

                let request = GetSessionTokenRequest {
                    duration_seconds: None,
                    serial_number: Some(mfa_serial.as_ref().unwrap().to_owned()),
                    token_code: Some(token_code),
                };
                let response = client.get_session_token(&request);
                match response {
                    Ok(response) => {
                        let credentials = response.credentials.unwrap();
                        println!("{:?}", credentials);
                        match save_credentials(credentials) {
                            Ok(_) => println!("Saved to credentials file."),
                            Err(err) => println!("Error saving credentials to file: {:?}", err)
                        };
                    },
                    Err(err) => panic!("Failed to get session token for profile {}: {:?}", profile_name, err),
                };
            }
        }

        pub fn show(&self, profile_name: &str) {
            println!("Showing config for profile {}...", profile_name);
            println!("{:?}", self.AWSConfig.profiles[profile_name]);
        }
    }

    pub fn create(profile_name: &str) {
        println!("Creating session for profile \"{}\"...", profile_name);

    }

    pub fn new() -> SessionHandler {
            SessionHandler {
                AWSConfig: AWSConfig {
                    profiles: HashMap::new()
                }
            }
    }

    pub fn refresh() {
        println!("Refreshing sessions...");
    }

    pub fn clean() {
        println!("Cleaning sessions...");
    }

    fn read_aws_config_file() -> String {
        let path = match env::home_dir() {
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

    fn split_config_file(aws_config: String) -> Vec<String> {
        let split = aws_config.split("\n\n");
        let mut profiles: Vec<String> = split.map(|s| s.to_string()).collect();;

        profiles.pop(); // Remove last element as it is always empty
        profiles
    }

    fn save_credentials(credentials: Credentials) -> Result<(), io::Error> {
        let mut aws_credentials_path = env::home_dir().unwrap().display().to_string();
        aws_credentials_path.push_str("/.aws/credentials"); 
        let mut file = OpenOptions::new()
                       .write(true)
                       .append(true)
                       .open(aws_credentials_path)
                       .unwrap();

        
        try!(file.write_all("Hi from Bluenine!\n".as_bytes()));
        Ok(())
    }
}
