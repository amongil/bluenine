#[macro_use]
extern crate clap;
use clap::App;
 
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("create") => println!("create!"),
        Some("show") => println!("show!"),
        Some("refresh") => println!("delete!"),
        Some("clean") => println!("clean!"),
        _ => println!("You must specify something")
    }
}
