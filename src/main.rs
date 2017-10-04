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
            let pName = sub_m.value_of("profile_name").unwrap();
            session_handler::create(pName);
        },
        ("show", _) => {
            session_handler::show()
        },
        ("refresh", _) => {
            session_handler::refresh()
        },
        ("clean",  _) => {
            session_handler::clean()
        },
        _ => return
    }
}
