use pwvlt::{get::handle_get, set::handle_set};
use clap::{App, Arg, ArgGroup};

fn main() {
    let matches = App::new("Password Vault")
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
        .group(
            ArgGroup::with_name("cmd")
                .required(true)
                .args(&["set", "get"]),
        )
        .get_matches();
    if let Some(values) = matches.values_of("get") {
        if let Err(err) = handle_get(values) {
            eprintln!("Failed to retrieve password: {}", err);
        }
    }
    if let Some(values) = matches.values_of("set") {
        handle_set(values).unwrap();
    }
}
