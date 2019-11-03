use clap::{App, Arg, ArgGroup};
use pwvlt::{get::get_password, set::set_password};

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
                .value_names(&["service", "username"])
                .min_values(2)
                .max_values(2),
        )
        .arg(
            Arg::with_name("set")
                .short("s")
                .long("set")
                .help("Set password for <service> <username>.")
                .value_names(&["service", "username"])
                .min_values(2)
                .max_values(2),
        )
        .group(
            ArgGroup::with_name("cmd")
                .required(true)
                .args(&["set", "get"]),
        )
        .get_matches();
    if let Some(mut values) = matches.values_of("get") {
        let service = values.next().unwrap();
        let username = values.next().unwrap();
        if let Err(err) = get_password(&service, &username) {
            eprintln!("Failed to retrieve password: {}", err);
        }
    }
    if let Some(mut values) = matches.values_of("set") {
        let service = values.next().unwrap();
        let username = values.next().unwrap();
        set_password(service, username).unwrap();
    }
}
