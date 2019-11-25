use clap::{App, Arg, ArgGroup};
use clipboard::{ClipboardContext, ClipboardProvider};

use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use pwvlt::vault::PasswordVault;

const DEFAULT_TIMEOUT: u8 = 7;

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
    let pv = PasswordVault::new();
    if let Some(mut values) = matches.values_of("get") {
        let service = values.next().unwrap();
        let username = values.next().unwrap();
        match pv.password(&service, &username) {
            Ok(password) => {
                let mut ctx: ClipboardContext =
                    ClipboardProvider::new().expect("Cannot initialize clipboard!");
                ctx.set_contents(password)
                    .expect("Cannot copy password to clipboard!");
                for i in (0..DEFAULT_TIMEOUT).rev() {
                    print!("Password copied to clipboard. ({}s)\r", i);
                    stdout().flush().unwrap();
                    sleep(Duration::from_secs(1));
                }
                println!();
                ctx.set_contents(String::new())
                    .expect("Cannot clear clipboard!");
            }
            Err(err) => {
                eprintln!("Failed to retrieve password: {}", err);
            }
        }
    }
    if let Some(mut values) = matches.values_of("set") {
        let service = values.next().unwrap();
        let username = values.next().unwrap();
        pv.set_password(&service, &username).expect("Failed to set password");
    }
}
