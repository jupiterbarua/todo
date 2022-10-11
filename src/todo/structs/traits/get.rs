use serde_json::Map;
use serde_json::value::Value;

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        let item: 

        println!("{} is being fetched", title);
    }
}