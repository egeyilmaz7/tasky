use clap::{Parser, Subcommand}; // use clap for creating cli commands
use core::fmt;
use serde::{Deserialize, Serialize}; // use serde for serialize and deserialize data structures
use serde_json::{from_str, to_string}; // serde for writing on json
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser)] // generate a parser function that turns cli input into your sturct field  
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)] // each command in this enum is a separate cli subcommand 
enum Commands {
    Add {
        #[arg(num_args = 1..)]
        task: Vec<String>,
    },

    Remove {
        task_number: usize,
    },

    List {},
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    text: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

fn read_from_json() -> Vec<Task> {
    let path = get_tasks_path();

    // open file if can't return an error massage
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // file doesn't exist yet - this is fine on first run!
            return Vec::new(); // silently return empty vec
        }
        Err(e) => {
            // this is an actual error (permissions, etc)
            eprintln!("Error opening file: {e}");
            return Vec::new();
        }
    };

    // read the contents from the file you just opened
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Failed to read file: {e}");
        return Vec::new();
    }

    // deserialize the contents
    let contents_deser: Vec<Task> = match from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to deserialze {e}");
            return Vec::new();
        }
    };

    // return the deserialized contents to use in write_from_json
    contents_deser
}

fn write_to_json(task: Vec<Task>) {
    // create file
    let path = get_tasks_path();
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file: {e}");
            return;
        }
    };

    // serialize the task into string
    let task_ser = match to_string(&task) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to serialize {e}");
            return;
        }
    };

    // write serialized task
    if let Err(e) = file.write_all(task_ser.as_bytes()) {
        eprintln!("Failed to write file: {e}");
    }
}

fn get_tasks_path() -> PathBuf {
    let home = dirs::home_dir().expect("Could not found home directory"); // get home directory
    let tasky_dir = home.join(".config").join("tasky"); // get tasky directory

    std::fs::create_dir_all(&tasky_dir).expect("Could not create tasky directory"); // create tasky directory 
    tasky_dir.join("tasks.json") // return path
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task } => {
            let task_text = task.join(" "); // this is required because of cleaner command writeing (not "" requirement)

            let mut tasks = read_from_json(); // read json

            // push the new task into json
            let new_task = Task {
                text: task_text.clone(),
            };
            tasks.push(new_task);

            // write it in json
            write_to_json(tasks);

            println!("✓ Task added: {task_text}");
        }

        Commands::Remove { task_number } => {
            let mut tasks = read_from_json();

            // error handling for user inputting 0 or 999 as task_num
            if task_number == 0 || task_number > tasks.len() {
                eprintln!("Error: Task {task_number} doesn't exist");
                return;
            }

            // for index handling
            let removed_task = task_number - 1;

            // remove by index
            tasks.remove(removed_task);

            // write it in json
            write_to_json(tasks);

            println!("✓ Task {task_number} removed");
        }

        Commands::List {} => {
            let tasks = read_from_json(); // returns Vec<Task>

            if tasks.is_empty() {
                println!("No tasks yet! Add one with: tasky add <task>");
            } else {
                // print list with a for loop and enumarate iterator consuming methods
                println!("All Tasks:\n");
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}. {}", i + 1, task);
                }
                println!();
            }
        }
    }
}
