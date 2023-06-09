use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

use crate::write_to_file;

use super::super::enums::TaskStatus;

pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        println!("{} is being set to done", title);

        state.insert(title.to_string(), json!(TaskStatus::DONE.stringify()));
        write_to_file("./state.json", state);

        println!("\n\n{} is being set to done", title);
    }

    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        println!("{} is being set to pending", title);

        state.insert(title.to_string(), json!(TaskStatus::PENDING.stringify()));
        write_to_file("./state.json", state);

        println!("\n\n{} is being set to pending", title);
    }
}
