use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

use crate::write_to_file;

pub trait Create {
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        println!("{} is being created", title);

        state.insert(title.to_string(), json!(status));
        write_to_file("./state.json", state);

        println!("\n\n{} is being created", title);
    }
}
