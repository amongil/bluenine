extern crate rusoto_core;
extern crate rusoto_sts;
extern crate rusoto_dynamodb;

pub mod SessionHandler {
    use rusoto_core::{DefaultCredentialsProvider, Region};
    use rusoto_core::{default_tls_client, ProfileProvider, ProvideAwsCredentials};
    use rusoto_sts::{StsClient, StsAssumeRoleSessionCredentialsProvider};
    use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

    pub fn create(profile_name: &str) {
        println!("Creating session for profile \"{}\"...", profile_name);

        let mut profile = ProfileProvider::new().unwrap();
        profile.set_profile(profile_name);

        let sts = StsClient::new(default_tls_client().unwrap(), profile, Region::EuWest1);
        let provider = StsAssumeRoleSessionCredentialsProvider::new(
            sts,
            "arn:aws:iam::247901982038:role/CloudreachAdminRole".to_owned(),
            "default".to_owned(),
            None, None, None, None
        );
        let client = DynamoDbClient::new(default_tls_client().unwrap(), profile, Region::EuWest1);
        let list_tables_input: ListTablesInput = Default::default();

        match client.list_tables(&list_tables_input) {
            Ok(output) => {
                match output.table_names {
                    Some(table_name_list) => {
                        println!("Tables in database:");

                        for table_name in table_name_list {
                            println!("{}", table_name);
                        }
                    }
                    None => println!("No tables in database!"),
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
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
}
