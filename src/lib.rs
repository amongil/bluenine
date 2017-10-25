extern crate regex;

extern crate rusoto_core;
extern crate rusoto_sts;
extern crate rusoto_dynamodb;

pub mod SessionHandler {
    use std::fs::{File, OpenOptions};
    use std::io::BufReader;
    use std::io;
    use std::io::prelude::*;
    use std::io::{stdin, stdout};
    use std::env;
    use rusoto_core::{ProfileProvider, Region, default_tls_client};
    use rusoto_sts::{Sts, StsClient, GetSessionTokenRequest, GetSessionTokenResponse, 
        GetSessionTokenError, AssumeRoleRequest, AssumeRoleResponse, AssumeRoleError, Credentials};

    use std::collections::HashMap;

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
    }
    
    impl AWSConfig {
        fn save(&self) -> Result<(), io::Error> {
            println!("Saving config to disk");
            let mut bluenine_config_path = env::home_dir().unwrap().display().to_string();
            bluenine_config_path.push_str("/.aws/config"); 
            let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(bluenine_config_path)
                        .unwrap();

            let mut prof = String::new();
            for (name, profile) in &self.profiles {
                prof.push_str(&format!("[profile {}]\n", name));

                let source_profile = &profile.source_profile;
                match source_profile {
                    &Some(ref source_profile) => {
                        prof.push_str(&format!("source_profile = {}\n", source_profile));
                    },
                    &None => {},
                };

                let region = &profile.region;
                match region {
                    &Some(ref region) => {
                        prof.push_str(&format!("region = {}\n", region));
                    },
                    &None => {},
                };

                let output = &profile.output;
                match output {
                    &Some(ref output) => {
                        prof.push_str(&format!("output = {}\n", output));
                    },
                    &None => {},
                };

                let role_arn = &profile.role_arn;
                match role_arn {
                    &Some(ref role_arn) => {
                        prof.push_str(&format!("role_arn = {}\n", role_arn));
                    },
                    &None => {},
                };

                let mfa_serial = &profile.mfa_serial;
                match mfa_serial {
                    &Some(ref mfa_serial) => {
                        prof.push_str(&format!("mfa_serial = {}\n", mfa_serial));
                    },
                    &None => {},
                };

                prof.push_str("\n");
            }

            try!(file.write_all(prof.as_bytes()));
            Ok(())
        }

        fn get_profile(&self, name: &str) -> &AWSProfile {
            if (&self.profiles).contains_key(name) {
                &self.profiles[name]
            }
            else {
                panic!("Could not find profile {}", name);
            }
        }

        fn contains_profile(&self, name: &str) -> bool {
            if (&self.profiles).contains_key(name) {
                true
            }
            else {
                false
            }
        }
    }

    pub fn create(profile_name: &str) {
        println!("Loading config file");
        let aws_config = load_config();
        
        let mut session_name = profile_name.clone().to_string();
        session_name.push_str("-session");
        if aws_config.contains_profile(&session_name) {
            println!("Session {} for profile {} already exists", session_name, profile_name);
        }

        println!("Creating session for profile \"{}\"...", profile_name);
        let aws_profile = aws_config.get_profile(profile_name);

        // Check if this is a child profile
        let source_profile = &aws_profile.source_profile;
        match source_profile {
            &Some(ref source_profile) => {
                println!("This is a child profile");
                let mut session_name = source_profile.clone().to_string();
                session_name.push_str("-session");
                if aws_config.contains_profile(&session_name) {
                    println!("Session {} for profile {} already exists", session_name, source_profile);
                    let mut provider = ProfileProvider::new().unwrap();
                    provider.set_profile(session_name);
                    let client = StsClient::new(default_tls_client().unwrap(), provider, Region::EuWest1);

                    let mut role_arn = &aws_profile.role_arn;
                    let role_arn_string = role_arn.as_ref().unwrap();
                    let v: Vec<&str> = role_arn_string.split('/').collect();

                    let request = AssumeRoleRequest {
                        duration_seconds: None,
                        external_id: None,
                        policy: None,
                        role_arn: role_arn_string.to_owned(),
                        role_session_name: v[1].to_owned(),
                        serial_number: None,
                        token_code: None,
                    };
                    let response = client.assume_role(&request);
                    match response {
                        Ok(response) => {
                            match save_profile(profile_name, &aws_profile) {
                                Ok(_) => println!("Saved profile to config file."),
                                Err(err) => println!("Error saving profile config to file: {:?}", err)
                            };
                            let credentials = response.credentials.unwrap();
                            match save_credentials(profile_name, credentials) {
                                Ok(_) => println!("Saved to credentials file."),
                                Err(err) => println!("Error saving credentials to file: {:?}", err)
                            };
                        },
                        Err(err) => panic!("Failed to get session token for profile {}: {:?}", profile_name, err),
                    };
                }
                else {
                    println!("Parent profile does not exist. Will have to create it");
                    return;
                }
            },
            &None => {
                let mut provider = ProfileProvider::new().unwrap();
                provider.set_profile(profile_name);
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
                            match save_profile(profile_name, &aws_profile) {
                                Ok(_) => println!("Saved profile to config file."),
                                Err(err) => println!("Error saving profile config to file: {:?}", err)
                            };
                            let credentials = response.credentials.unwrap();
                            match save_credentials(profile_name, credentials) {
                                Ok(_) => println!("Saved to credentials file."),
                                Err(err) => println!("Error saving credentials to file: {:?}", err)
                            };
                        },
                        Err(err) => panic!("Failed to get session token for profile {}: {:?}", profile_name, err),
                    };
                }
            },
        };
    }
    pub fn show() {
        println!("Showing bluenine sessions...");
        println!("Loading config file");
        let aws_config = load_config();
        println!("Active sessions:\n");

        for (name, aws_profile) in aws_config.profiles {
            if name.contains("-session") {
                println!("{}", name);
            }
        }
    }

    pub fn clean(profile_name: &str) {
        println!("Loading config file");
        let mut aws_config = load_config();
        let mut session_name = profile_name.clone().to_string();
        session_name.push_str("-session");

        if aws_config.contains_profile(&session_name) {
            println!("Cleaning session {}", session_name);
            aws_config.profiles.remove(&session_name);
            aws_config.save();
            // Remove credentials also
            remove_credentials(&session_name);
        }
        else {
            println!("No profiles to clean");
        }
    }

    fn load_config() -> AWSConfig {
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
                    _ => (),
                }
            }

            if profile_name != "" {
                aws_config.profiles.insert(profile_name.clone(), aws_profile);
            }
        }

        aws_config
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

    fn read_aws_credentials_file() -> String {
        let path = match env::home_dir() {
            Some(path) => path,
            None => panic!("Could not retrieve user's home directory."),
        };
        let config_file_path = format!("{}/.aws/credentials", path.display());

        let f = File::open(config_file_path).expect("Could not find AWS credentials file.");
        let mut buf_reader = BufReader::new(f);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("Found credentials file but could not read it.");

        contents
    }

    fn split_config_file(aws_config: String) -> Vec<String> {
        let split = aws_config.split("\n\n");
        let mut profiles: Vec<String> = split.map(|s| s.to_string()).collect();;

        profiles.pop(); // Remove last element as it is always empty
        profiles
    }

    fn save_credentials(profile_name: &str, credentials: Credentials) -> Result<(), io::Error> {
        let mut aws_credentials_path = env::home_dir().unwrap().display().to_string();
        aws_credentials_path.push_str("/.aws/credentials"); 
        let mut file = OpenOptions::new()
                       .write(true)
                       .append(true)
                       .open(aws_credentials_path)
                       .unwrap();

        let mut creds = String::new();
        creds.push_str(&format!("[{}-session]\n", profile_name));
        creds.push_str(&format!("aws_access_key_id = {}\n", credentials.access_key_id));
        creds.push_str(&format!("expiration = {}\n", credentials.expiration));
        creds.push_str(&format!("aws_secret_access_key = {}\n", credentials.secret_access_key));
        creds.push_str(&format!("aws_session_token = {}\n\n", credentials.session_token));
        try!(file.write_all(creds.as_bytes()));
        Ok(())
    }

    fn remove_credentials(profile_name: &str) -> Result<(), io::Error> {
        println!("Saving credentials to disk");
        let aws_credentials_file = read_aws_credentials_file();
        let credentials = split_config_file(aws_credentials_file);
        let mut aws_credentials_path = env::home_dir().unwrap().display().to_string();
        aws_credentials_path.push_str("/.aws/credentials"); 
        let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(aws_credentials_path)
                .unwrap();
        
        let mut creds = String::new();
        for credential in credentials {
            let split = credential.split("\n");
            let lines: Vec<String> = split.map(|s| s.to_string()).collect();

            // Get the profile name from the first line
            let profile_line = lines[0].to_owned();
            if profile_line == format!("[{}]", profile_name) {
                continue;
            }
            creds.push_str(&credential);
            creds.push_str("\n\n");
        }
        try!(file.write_all(creds.as_bytes()));
        Ok(())
    }

    fn save_profile(profile_name: &str, aws_profile: &AWSProfile) -> Result<(), io::Error> {
        let mut aws_config_path = env::home_dir().unwrap().display().to_string();
        aws_config_path.push_str("/.aws/config"); 
        let mut file = OpenOptions::new()
                       .write(true)
                       .append(true)
                       .open(aws_config_path)
                       .unwrap();

        let region = &aws_profile.region;
        match region {
            &Some(ref region) => {
                let mut prof = String::new();
                prof.push_str(&format!("[profile {}-session]\n", profile_name));
                prof.push_str(&format!("region = {}\n\n", region));
                try!(file.write_all(prof.as_bytes()));
                Ok(())
            },
            &None => {
                let region = "us-east-1";
                let mut prof = String::new();
                prof.push_str(&format!("[profile {}-session]\n", profile_name));
                prof.push_str(&format!("region = {}\n\n", region));
                try!(file.write_all(prof.as_bytes()));
                Ok(())
            },
        }
    }
}
