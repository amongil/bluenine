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

    struct AWSConfig {
        profiles: Option<Vec<AWSProfile>>,
    }

    struct AWSProfile {
        source_profile: Option<String>,
        region: Option<String>,
        output: Option<String>,
        role_arn: Option<String>,
        mfa_serial: Option<String>
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

    pub fn show() {
        println!("Showing sessions...");
        let aws_config_file = read_aws_config_file();
        println!("{}", aws_config_file);

        println!("Spliting...");
        let profiles = split_config_file(aws_config_file);
        for profile in &profiles {
            println!("{}", profile);
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
        let profiles: Vec<String> = split.map(|s| s.to_string()).collect();;

        profiles
    }
}
