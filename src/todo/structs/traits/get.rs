use serde_json::Map;
use serde_json::value::Value;

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("\n\nStatus: {}", result);
            }, 
            None =>  println!("Item: {} was not found", title)
        }

        println!("{} is being fetched\n\n", title);
    }
}