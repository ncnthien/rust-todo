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

fn create_db() -> Result<(), String> {
    let Ok(mut file) = fs::File::create("db.json") else {
        return Err("Cannot create db.json file".to_string())
    };

    let db = json!({
        "current_id": 0,
        "items": []
    });

    let _ = file.write_all(db.to_string().as_bytes()); 

    Ok(())
}

fn get_db() -> Result<Database, String> {
    let Ok(db_json) = fs::read_to_string("db.json") else {
        return Err("Cannot read db.json".to_string())
    };

    let Ok(db) = serde_json::from_str(&db_json) else {
        return Err("Cannot parse json into struct".to_string())
    };

    Ok(db)
}

fn write_db(db: &Database) -> Result<&str, String> {
    let Ok(new_db_json) = serde_json::to_string_pretty(db) else {
        return Err("Cannot convert db to json".to_string())
    };

    let _ = fs::write("db.json", new_db_json);
    Ok("Write db success!")
}

fn add(desc: &String) -> Result<&str, String> {
    if !is_db_exists() {
        create_db()?;
    }

    let mut db = get_db().unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });
    let item = Item { done: false, desc: desc.to_owned(), id: db.current_id + 1 };

    db.items.push(item);
    db.current_id += 1;

    write_db(&db)?;

    Ok(desc.as_str())
}

fn list() -> Result<(), String> {
    if !is_db_exists() {
        create_db()?;
    }

    let db = get_db()?;

    for item in db.items {
        let mark = if item.done { "x" } else { " " };
        println!("[{}] {} - {}", mark, item.id, item.desc);
    }

    Ok(())
}

fn remove(id: &i8) -> Result<&i8, String> {
    let mut db = get_db()?;

    if let Some(removed_item_index) = db.items.iter().position(|item| &item.id == id) {
        db.items.remove(removed_item_index);
    };
    write_db(&db)?;

    Ok(id)
}

fn done(id: &i8) -> Result<(&i8, bool), String> {
    let mut db = get_db()?;

    let Some(done_item_index) = db.items.iter().position(|item| {
        &item.id == id
    }) else {
        return Err(format!("Item with id {} is not found", id))
    };

    let new_done = !db.items[done_item_index].done;
    db.items[done_item_index].done = new_done;
    write_db(&db)?;

    Ok((id, new_done))
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Add { description } => {
            let added_desc = add(description).unwrap_or_else(|error| {
                eprintln!("Error found: {error}");
                process::exit(1);
            });
            println!("Added '{}'", added_desc);
        },
        Command::Remove { id } => {
            let removed = remove(id).unwrap_or_else(|error| {
                eprintln!("Error found: {error}");
                process::exit(1);
            });
            println!("Removed item with id {}", removed);
        },
        Command::Done { id } => {
            let (id, new_done) = done(id).unwrap_or_else(|error| {
                eprintln!("Error found: {error}");
                process::exit(1);
            });
            let status = if new_done { "completed" } else { "uncompleted" };
            println!("Mark item with id {} to {}", id, status);
        },
        Command::List => {
            let _ = list().unwrap_or_else(|error| {
                eprintln!("Error found: {error}");
                process::exit(1);
            });
        }
    };
}
