use clap::{Arg, Command};
mod db;
use db::Db;

fn main() {
    let matches = Command::new("Rust NoSQL DB")
        .version("0.1.0")
        .author("Emmanuel Samuel")
        .about("A simple in-memory NoSQL database")
        .subcommand_required(true) // Ensures that a subcommand must be provided
        .subcommand(
            Command::new("set")
                .about("Sets a value for a specified key")
                .arg(Arg::new("KEY").required(true).index(1))
                .arg(Arg::new("VALUE").required(true).index(2)),
        )
        .subcommand(
            Command::new("get")
                .about("Gets a value by key")
                .arg(Arg::new("KEY").required(true).index(1)),
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes a key and its value")
                .arg(Arg::new("KEY").required(true).index(1)),
        )
        .get_matches();


    let mut db = Db::load().unwrap_or_else(|_| Db::new());

    match matches.subcommand() {
        Some(("set", sub_m)) => {
            println!("Running 'set' command"); // Debug print
            let key = sub_m.value_of("KEY").unwrap();
            let value = sub_m.value_of("VALUE").unwrap();
            db.set(key.to_string(), value.to_string());
            db.save().expect("Failed to save data");
        },
        Some(("get", sub_m)) => {
            println!("Running 'get' command"); // Debug print
            let key = sub_m.value_of("KEY").unwrap();
            if let Some(value) = db.get(key) {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        },
        Some(("delete", sub_m)) => {
            println!("Running 'delete' command"); // Debug print
            let key = sub_m.value_of("KEY").unwrap();
            db.delete(key);
            db.save().expect("Failed to save data");
        },
        _ => {
            eprintln!("Unknown command");
            std::process::exit(1);
        }
    }
    
}

