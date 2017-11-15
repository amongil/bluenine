#[macro_use]

extern crate clap;
use clap::App;
 
extern crate bluenine;
use bluenine::session_handler;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("create", Some(sub_m)) => {
            let profile_name = sub_m.value_of("profile_name").unwrap();
            session_handler::create(profile_name);
            session_handler::show();
        },
        ("show", _)=> {
            session_handler::show();
        },
        ("refresh", Some(sub_m)) => {
            match sub_m.value_of("profile_name") {
                Some(profile_name) => {
                    session_handler::clean_profile(profile_name);
                    session_handler::create(profile_name);
                    session_handler::show();  
                },
                None => {
                    session_handler::refresh_all_profiles();
                    session_handler::show();
                },
            }
        },
        ("clean", Some(sub_m)) => {
            match sub_m.value_of("profile_name") {
                Some(profile_name) => session_handler::clean_profile(profile_name),
                None => session_handler::clean_all_profiles(),
            }
        },
        _ => return
    }
}
