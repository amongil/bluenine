#[macro_use]

extern crate clap;
use clap::App;
 
extern crate bluenine;
use bluenine::SessionHandler;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut session_handler = SessionHandler::new();
    session_handler.load_config();
    println!("Successfully loaded config!");

    match matches.subcommand() {
        ("create", Some(sub_m)) => {
            let profile_name = sub_m.value_of("profile_name").unwrap();
            session_handler.create(profile_name);
        },
        ("show", Some(sub_m)) => {
            let profile_name = sub_m.value_of("profile_name").unwrap();
            session_handler.show(profile_name);
        },
        ("refresh", _) => {
            SessionHandler::refresh()
        },
        ("clean",  _) => {
            SessionHandler::clean()
        },
        _ => return
    }
}
