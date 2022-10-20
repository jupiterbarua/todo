mod todo;
use todo::ItemTypes;
use todo::to_do_factory;
use todo::structs::traits::create::Create;
mod state;
mod process;
use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    
    let state: Map<String, Value> = read_file("./state.json");
    let status: String;
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"',"");
        },
        None => { 
            status =String::from("pending");
        }
    };
    let item = to_do_factory(&status, title.to_string()).expect(&status);
    process::process_input(item, command.to_string(), &state);
}
