extern crate serde_json;
extern crate app_dirs;

use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use app_dirs::{AppInfo, AppDataType};

const APP_INFO: AppInfo = AppInfo{name: "rmbr", author: "amatho"};
const APP_TYPE: AppDataType = AppDataType::UserData;

type StoreMap = HashMap<String, String>;

fn main() -> io::Result<()> {
    let store_file = app_dirs::app_root(APP_TYPE, &APP_INFO)
        .expect("Could not read user data directory")
        .join("store");
    let (args_result, debug) = get_args();
    let args = match args_result {
        Ok(a) => a,
        Err(e) => {
            println!("{}", e);
            vec![]
        }
    };
    let mut store = load_store(&store_file, debug);

    run_command(&args, &mut store);

    update_store(&store_file, store)?;
    Ok(())
}

fn run_command(args: &Vec<String>, store: &mut StoreMap) {
    match args[0].as_ref() {
        "help" => help(),
        "new" => new(&args, store),
        "remove" => remove(&args, store),
        "list" => list(store),
        _ => help()
    }
}

fn new(args: &Vec<String>, store: &mut StoreMap) {
    if args.len() < 3 {
        println!("You must specify both a name and a description.");
        return;
    }

    store.insert(args[1].clone(), args[2].clone());
}

fn remove(args: &Vec<String>, store: &mut StoreMap) {
    if args.len() < 2 {
        println!("You must specify the name for the item to remove.");
        return;
    }

    store.remove(&args[1]);
}

fn list(store: &StoreMap) {
    println!("You have the following stuff to remember:\n");
    for (key, val) in store {
        println!("--- {} ---\n{}\n", key, val);
    }
}

fn help() {
    println!("
    rmbr.

    Usage:
      rmbr new <name> <description>
      rmbr remove <name>
      rmbr list
      rmbr help

    Options:
      --debug   Show extra debug information.
    ");
}

fn load_store(path: &PathBuf, debug: bool) -> StoreMap {
    let mut f: File;
    let mut existing = false;
    
    match File::open(path) {
        Ok(file) => {
            f = file;
            existing = true;
        },
        _ => {
            f = File::create(path).expect("Could not create store file");
            f.write(b"{}\n").expect("Could not write to store file");
        }
    };

    if existing == false {
        return HashMap::new();
    }

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read store file");
    let map: StoreMap = serde_json::from_str(&contents).expect("Could not parse the store file");
    
    if debug {
        println!("Loaded store: {:?}", map)
    }

    map
}

fn update_store(path: &PathBuf, map: StoreMap) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    let mut bytes = serde_json::to_string(&map)?.into_bytes();
    bytes.push('\n' as u8);
    f.write_all(&bytes)?;
    Ok(())
}

fn get_args() -> (Result<Vec<String>, &'static str>, bool) {
    let args_vec: Vec<String> = env::args().collect();
    if args_vec.len() >= 2 {
        let debug = &args_vec[1] == "--debug";
        (Ok(args_vec[1..].to_vec()), debug)
    } else {
        (Err("Not enough arguments"), false)
    }
}
