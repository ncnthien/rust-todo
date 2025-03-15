use clap::{Parser, Subcommand};
use std::process;
use std::fs;
use serde::{Serialize, Deserialize};
use std::io::Write;
use serde_json::json;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add {
        description: String
    },
    Remove {
        id: i8
    },
    Done {
        id: usize
    },
    List
}


#[derive(Serialize, Deserialize)]
struct Item {
    id: i8,
    done: bool,
    desc: String
}

#[derive(Serialize, Deserialize)]
struct Database {
    current_id: i8,
    items: Vec<Item>
}

fn is_db_exists() -> bool {
    Path::new("db.json").exists()
}

fn create_db() {
    let create_file_result = fs::File::create("db.json");

    if let Ok(mut file) = create_file_result {
        let db = json!({
            "current_id": 1,
            "items": []
        });

        file.write_all(db.to_string().as_bytes()).expect("Cannot write data to db.json for the first time");
    } else {
        eprintln!("Cannot create db.json file");
        process::exit(1);
    }
}

fn add<'a>(desc: &String) -> Result<&'a str, std::io::Error> {
    if !is_db_exists() {
        create_db();
    }

    let db_json = fs::read_to_string("db.json")?;
    let mut db: Database = serde_json::from_str(&db_json)?;
    let item = Item { done: false, desc: desc.to_owned(), id: db.current_id + 1 };

    db.items.push(item);
    db.current_id += 1;

    let new_db_json = serde_json::to_string_pretty(&db)?;
    fs::write("db.json", new_db_json)?;

    Ok("Success")
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Add { description } => {
            let message = add(description).unwrap_or_else(|err| {
                eprintln!("Error found: {err}");
                process::exit(1);
            });
            println!("{}", message);
            ()
        },
        Command::Remove { id } => {
            println!("remove");
        },
        Command::Done { id } => {
            println!("done");
        },
        Command::List => {
            println!("list");
        }
    };
}
