mod todo;
use todo::ItemTypes;
use todo::to_do_factory;
use todo::structs::traits::create::Create;
mod state;
use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "make");
    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => 
            item.create(&item.super_struct.title),
        ItemTypes::Done(item) => 
            println!("It is a done item with title {}", item.super_struct.title)    
    }

    let args: Vec<String> = env::args().collect();
    println!("{:?} ", args);
    let status: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String, Value> = read_file("./state.json");
    println!("{:?} and {:?}", status, title);
    state.insert(title.to_string(), json!(status));
    println!("{:?}", state);
    write_to_file("./state.json", &mut state);
}
