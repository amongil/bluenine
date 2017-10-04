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
        },
        ("show", _) => {
            SessionHandler::show()
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
