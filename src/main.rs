mod processes;
mod state;
mod todo;

// use todo::structs::done::Done;

// use todo::structs::pending::Pending;

use processes::process_input;
use todo::enums::TaskStatus;
use todo::todo_factory;
use todo::ItemTypes;

// import the traits into the file that is using them
use crate::todo::traits::delete::Delete;
use crate::todo::traits::edit::Edit;
use crate::todo::traits::get::Get;

use serde_json::value::Value;
use serde_json::{json, Map};
use state::{read_file, write_to_file};
use std::env;

fn main() {
    // let done = Done::new("shopping");

    // println!("{}", done.super_struct.title);
    // println!("{}", done.super_struct.status.stringify());

    // let pending = Pending::new("laundry");
    // println!("{}", pending.super_struct.title);
    // println!("{}", pending.super_struct.status.stringify());

    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    // let status: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file("./state.json");
    let status: String;

    match &state.get(*&title) {
        Some(result) => status = result.to_string().replace('\"', ""),
        None => status = "pending".to_string(),
    }
    let item = todo_factory(title, TaskStatus::from_string(status.to_uppercase()));
    process_input(item, command.to_string(), &state);

    // println!("Before operation {:?}", state);

    // state.insert(title.to_string(), json!(status));

    // println!("After operation {:?}", state);

    // write_to_file("./state.json", &mut state);

    // let todo_item = todo_factory("washing", TaskStatus::DONE);

    // match todo_item {
    //     ItemTypes::Done(item) => {
    //         // println!("{}", item.super_struct.status.stringify());
    //         // println!("{}", item.super_struct.title);
    //         item.get(&item.super_struct.title, &mut state);
    //         // item.delete(&item.super_struct.title);
    //     }
    //     ItemTypes::Pending(item) => {
    //         // println!("{}", item.super_struct.status.stringify());
    //         // println!("{}", item.super_struct.title);
    //         item.get(&item.super_struct.title, &mut state);
    //         // item.set_to_done(&item.super_struct.title)
    //     }
    // }
}
