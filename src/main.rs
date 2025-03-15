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
            "current_id": 0,
            "items": []
        });

        file.write_all(db.to_string().as_bytes()).expect("Cannot write data to db.json for the first time");
    } else {
        eprintln!("Cannot create db.json file");
        process::exit(1);
    }
}

fn get_db() -> Result<Database, std::io::Error> {
    let db_json = fs::read_to_string("db.json")?;
    let db: Database = serde_json::from_str(&db_json)?;
    Ok(db)
}

fn write_db(db: &Database) -> Result<&str, std::io::Error> {
    let new_db_json = serde_json::to_string_pretty(db)?;
    fs::write("db.json", new_db_json)?;
    Ok("Write db success!")
}

fn add<'a>(desc: &String) -> Result<&'a str, std::io::Error> {
    if !is_db_exists() {
        create_db();
    }

    let mut db = get_db()?;
    let item = Item { done: false, desc: desc.to_owned(), id: db.current_id + 1 };

    db.items.push(item);
    db.current_id += 1;

    write_db(&db)?;

    Ok("Success")
}

fn list() -> () {
    if !is_db_exists() {
        create_db();
    }

    let db = get_db().unwrap_or_else(|error| {
        eprintln!("Issue found: {}", error);
        process::exit(1);
    });

    for item in db.items {
        let mark = if item.done { "x" } else { " " };
        println!("[{}] {} - {}", mark, item.id, item.desc);
    }
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
            list();
        }
    };
}
