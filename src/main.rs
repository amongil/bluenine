#[macro_use]

extern crate clap;
use clap::App;
 
extern crate bluenine;
use bluenine::SessionHandler;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("create", Some(sub_m)) => {
            let profile_name = sub_m.value_of("profile_name").unwrap();
            SessionHandler::create(profile_name);
            SessionHandler::show();
        },
        ("show", _)=> {
            SessionHandler::show();
        },
        ("refresh", Some(sub_m)) => {
            match sub_m.value_of("profile_name") {
                Some(profile_name) => {
                    SessionHandler::clean_profile(profile_name);
                    SessionHandler::create(profile_name);
                    SessionHandler::show();  
                },
                None => {
                    SessionHandler::refresh_all_profiles();
                    SessionHandler::show();
                },
            }
        },
        ("clean", Some(sub_m)) => {
            match sub_m.value_of("profile_name") {
                Some(profile_name) => SessionHandler::clean_profile(profile_name),
                None => SessionHandler::clean_all_profiles(),
            }
        },
        _ => return
    }
}
