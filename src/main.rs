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
        id: i8
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

fn create_db<'a>() -> Result<(), &'a str> {
    let Ok(mut file) = fs::File::create("db.json") else {
        return Err("Cannot create db.json file")
    };

    let db = json!({
        "current_id": 0,
        "items": []
    });

    let _ = file.write_all(db.to_string().as_bytes()); 

    Ok(())
}

fn get_db<'a>() -> Result<Database, &'a str> {
    let Ok(db_json) = fs::read_to_string("db.json") else {
        return Err("Cannot read db.json")
    };

    let Ok(db) = serde_json::from_str(&db_json) else {
        return Err("Cannot parse json into struct")
    };

    Ok(db)
}

fn write_db<'a>(db: &Database) -> Result<&'a str, &str> {
    let Ok(new_db_json) = serde_json::to_string_pretty(db) else {
        return Err("Cannot convert db to json")
    };

    let _ = fs::write("db.json", new_db_json);
    Ok("Write db success!")
}

fn add<'a>(desc: &String) -> Result<&'a str, &str> {
    if !is_db_exists() {
        create_db().unwrap();
    }

    let mut db = get_db().unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });
    let item = Item { done: false, desc: desc.to_owned(), id: db.current_id + 1 };

    db.items.push(item);
    db.current_id += 1;

    let _ = write_db(&db);

    Ok("Add success!")
}

fn list<'a>() -> Result<(), &'a str> {
    if !is_db_exists() {
        create_db().unwrap();
    }

    let db = get_db().unwrap();

    for item in db.items {
        let mark = if item.done { "x" } else { " " };
        println!("[{}] {} - {}", mark, item.id, item.desc);
    }

    Ok(())
}

fn remove(id: &i8) -> Result<&i8, &str> {
    let mut db = get_db().unwrap();

    if let Some(removed_item_index) = db.items.iter().position(|item| &item.id == id) {
        db.items.remove(removed_item_index);
    };

    println!("Remove item with id {} success!", id);

    Ok(id)
}

fn done(id: &i8) -> Result<(), &str> {

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Add { description } => {
            let message = add(description).unwrap_or_else(|error| {
                eprintln!("Error found: {error}");
                process::exit(1);
            });
            println!("{}", message);
            ()
        },
        Command::Remove { id } => {
            remove(id).unwrap_or_else(|error| {
                eprintln!("Error found: {error}");
                process::exit(1);
            });
        },
        Command::Done { id } => {
            done(id).unwrap_or_else(|error| {
                eprintln!("Error found: {error}");
                process::exit(1);
            });
        },
        Command::List => {
            let _ = list().unwrap_or_else(|error| {
                eprintln!("Error found: {error}");
                process::exit(1);
            });
        }
    };
}
