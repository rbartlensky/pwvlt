use prettytable::{cell, row, Table};

use pwvlt::{PwvltError, Slot};

use std::fmt::Display;
use std::io::{self, stdout, BufRead, Write};
use std::ops::Sub;
use std::str::FromStr;

pub fn print_slots(slots: &[Slot]) -> Result<(), PwvltError> {
    print!("Retrieving slots...\r");
    stdout().flush().unwrap();
    let mut table = Table::new();
    table.add_row(row!["Slot", "Service", "Username"]);
    slots
        .iter()
        .enumerate()
        .for_each(|(slot, Slot { service, username })| {
            table.add_row(row![slot.to_string(), service, username]);
        });
    table.printstd();
    Ok(())
}

pub fn looping_prompt<T>(item: &str, max_val: T) -> T
where
    T: Ord + Sub + Display + FromStr,
{
    loop {
        let item_val = prompt_string(format!("Select {} (0-{})", item, max_val));
        if let Ok(item_val) = item_val.parse::<T>() {
            if item_val <= max_val {
                return item_val;
            }
            println!("Invalid {} number: {}", item, item_val);
        } else {
            println!("Invalid {}: {}", item, item_val);
        }
    }
}

pub fn prompt_string<S: AsRef<str>>(message: S) -> String {
    print!("{}: ", message.as_ref());
    stdout().flush().unwrap();
    let mut item_val = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut item_val).unwrap();
    item_val.trim().into()
}
