use clap::{Parser, Subcommand};
use std::process;
use std::fs;
use serde::{Serialize, Deserialize};
use std::io::Write;
use serde_json::json;

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
struct Db {
    current_id: i8,
    items: Vec<Item>
}

fn get_db_file() -> fs::File {
    fs::File::open("db.json").unwrap_or_else(|_| {
        let create_file_result = fs::File::create("db.json");

        if let Ok(file) = create_file_result {
            file
        } else {
            eprintln!("Cannot create db.json file");
            process::exit(1);
        }
    })
}

fn add<'a>(desc: &String) -> Result<&'a str, String> {
    let item = Item { done: false, desc: desc.to_owned(), id: 1 };
    let item_json = serde_json::to_string(&item).unwrap_or_else(|err| {
        eprint!("Something went wrong on 'serde_json::to_string' with {{:?item}}!\nError: {err}");
        process::exit(1);
    });

    let db_file = get_db_file();


    Ok("Success")
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Add { description } => {
            add(description);
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
