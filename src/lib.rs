extern crate regex;

extern crate rusoto_core;
extern crate rusoto_sts;
extern crate rusoto_dynamodb;

pub mod SessionHandler {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::env;

    use rusoto_core::{DefaultCredentialsProvider, Region};
    use rusoto_core::{default_tls_client, ProfileProvider, ProvideAwsCredentials};
    use rusoto_sts::{StsClient, StsAssumeRoleSessionCredentialsProvider};
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

        pub fn show(&self, profile_name: &str) {
            println!("Showing config for profile {}...", profile_name);
            println!("{:?}", self.AWSConfig.profiles[profile_name]);
        }
    }

    pub fn create(profile_name: &str) {
        println!("Creating session for profile \"{}\"...", profile_name);

        // let mut profile = ProfileProvider::new().unwrap();
        // profile.set_profile(profile_name);

        // let sts = StsClient::new(default_tls_client().unwrap(), profile, Region::EuWest1);
        // let provider = StsAssumeRoleSessionCredentialsProvider::new(
        //     sts,
        //     "arn:aws:iam::247901982038:role/CloudreachAdminRole".to_owned(),
        //     "default".to_owned(),
        //     None, None, None, None
        // );
        // let client = DynamoDbClient::new(default_tls_client().unwrap(), profile, Region::EuWest1);
        // let list_tables_input: ListTablesInput = Default::default();

        // match client.list_tables(&list_tables_input) {
        //     Ok(output) => {
        //         match output.table_names {
        //             Some(table_name_list) => {
        //                 println!("Tables in database:");

        //                 for table_name in table_name_list {
        //                     println!("{}", table_name);
        //                 }
        //             }
        //             None => println!("No tables in database!"),
        //         }
        //     }
        //     Err(error) => {
        //         println!("Error: {:?}", error);
        //     }
        // }
    }
    pub fn new() -> SessionHandler {
            SessionHandler {
                AWSConfig: AWSConfig {
                    profiles: HashMap::new()
                }
            }
    }
    // pub fn show(profile_name: &str) {
    //     println!("Showing sessions...");
    // }

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

    // fn get_profile_names(aws_config: String) -> Vec<String>{
    //     let re = Regex::new(r"(?m)(\[profile+.+\])").unwrap();
    //     let caps = re.captures(&aws_config).unwrap();

    //     println!("{:?}", caps);
    //     let profile1 = caps.get(0).map_or("", |m| m.as_str());
    //     let profile2 = caps.get(1).map_or("", |m| m.as_str());

    //     let mut profiles = Vec::new();
    //     profiles.push(profile1.to_string());
    //     profiles.push(profile2.to_string());

    //     profiles
    // }

    fn split_config_file(aws_config: String) -> Vec<String> {
        let split = aws_config.split("\n\n");
        let mut profiles: Vec<String> = split.map(|s| s.to_string()).collect();;

        profiles.pop(); // Remove last element as it is always empty
        profiles
    }
}
