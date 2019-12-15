use clap::{App, Arg, ArgGroup, ArgMatches};
use clipboard::{ClipboardContext, ClipboardProvider};
use log::error;

use pwvlt::{util::prompt_string, vault::PasswordVault};

use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

mod config;
use config::write_config;
mod error;
use error::Error;

const DEFAULT_TIMEOUT: u8 = 7;

pub fn handle_get(
    pv: pwvlt::vault::PasswordVault,
    service: &str,
    username: &str,
) -> Result<(), Error> {
    let password = pv.password(&service, &username)?;
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(password)?;
    for i in (0..DEFAULT_TIMEOUT).rev() {
        print!("Password copied to clipboard. ({}s)\r", i);
        stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
    println!();
    ctx.set_contents(String::new())?;
    Ok(())
}

fn handle_args(args: ArgMatches) -> Result<(), Error> {
    let mut config = config::load_config()?;
    if let Some(mut values) = args.values_of("get") {
        let pv = PasswordVault::new(config);
        let service = values.next().unwrap();
        let username = match pv.default(&service) {
            Some(username) => username.to_string(),
            None => prompt_string(format!("Enter username for {}", service)),
        };
        handle_get(pv, service, &username)
    } else if let Some(mut values) = args.values_of("set") {
        let pv = PasswordVault::new(config);
        let service = values.next().unwrap();
        let username = match pv.default(&service) {
            Some(username) => username.to_string(),
            None => prompt_string(format!("Enter username for {}", service)),
        };
        pv.set_password(&service, &username).map_err(Error::from)
    } else if let Some(mut values) = args.values_of("set-default") {
        let service = values.next().unwrap();
        let username = values.next().unwrap();
        config.default.insert(service.into(), username.into());
        write_config(&config)
    } else {
        Ok(())
    }
}

fn main() {
    let matches = App::new("Password Vault")
        .version("1.0")
        .author("Robert B. <bartlensky.robert@gmail.com>")
        .about("Stores passwords on the local keyring or on a Nitrokey.")
        .arg(Arg::with_name("v").short("v").multiple(true))
        .arg(
            Arg::with_name("get")
                .short("g")
                .long("get")
                .help("Copy the password for <service> to the kill ring.")
                .value_names(&["service"]),
        )
        .arg(
            Arg::with_name("set")
                .short("s")
                .long("set")
                .help("Set password for <service>.")
                .value_names(&["service"]),
        )
        .arg(
            Arg::with_name("set-default")
                .short("d")
                .long("set-default")
                .help("Set the default <username> for <service>.")
                .value_names(&["service", "username"]),
        )
        .group(
            ArgGroup::with_name("cmd")
                .required(true)
                .args(&["set", "get", "set-default"]),
        )
        .get_matches();

    let level = match matches.occurrences_of("v") {
        0 => log::Level::Warn,
        1 => log::Level::Info,
        _ => log::Level::Trace,
    };
    simple_logger::init_with_level(level).expect("Failed to initialize logger...");

    if let Err(e) = handle_args(matches) {
        match e {
            Error::HomeNotFound => error!("Couldn't find home directory."),
            Error::Io(e) => error!("An IO error occurred: {}", e),
            Error::TomlDeserialize(e) => error!("Failed to deserialize config file: {}", e),
            Error::TomlSerialize(e) => error!("Failed to serialize config file: {}", e),
            Error::PassStore(e) => error!("Internal error occurred: {}", e),
        }
    }
}
