#[macro_use]
extern crate clap;
use clap::App;
 
extern crate bluenine;
use bluenine::session_handler;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("create") => session_handler::create(),
        Some("show") => session_handler::show(),
        Some("refresh") => session_handler::refresh(),
        Some("clean") => session_handler::clean(),
        _ => return
    }
}
