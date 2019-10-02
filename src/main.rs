use clap::*;
mod error;
mod get;
use get::handle_get;
mod set;
use set::handle_set;
mod config;
mod nitrokey;

fn main() {
    let matches = App::new("LocalNitro password store")
        .version("1.0")
        .author("Robert B. <bartlensky.robert@gmail.com>")
        .about("Stores passwords on the local keyring or on a Nitrokey.")
        .arg(
            Arg::with_name("get")
                .short("g")
                .long("get")
                .help("Copy the password for <service> to the kill ring.")
                .value_names(&["service", "[username]"])
                .min_values(1)
                .max_values(2),
        )
        .arg(
            Arg::with_name("set")
                .short("s")
                .long("set")
                .help("Set password for <service> <username>.")
                .value_names(&["service", "username", "[password]"])
                .min_values(2)
                .max_values(3),
        )
        .get_matches();
    if let Some(values) = matches.values_of("get") {
        handle_get(values).unwrap();
    }
    if let Some(values) = matches.values_of("set") {
        handle_set(values).unwrap();
    }
}
