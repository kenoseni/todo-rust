mod todo;

// use todo::structs::done::Done;

// use todo::structs::pending::Pending;

use todo::enums::TaskStatus;
use todo::todo_factory;
use todo::ItemTypes;

// import the traits into the file that is using them
use crate::todo::traits::delete::Delete;
use crate::todo::traits::edit::Edit;
use crate::todo::traits::get::Get;

fn main() {
    // let done = Done::new("shopping");

    // println!("{}", done.super_struct.title);
    // println!("{}", done.super_struct.status.stringify());

    // let pending = Pending::new("laundry");
    // println!("{}", pending.super_struct.title);
    // println!("{}", pending.super_struct.status.stringify());

    let todo_item = todo_factory("washing", TaskStatus::DONE);

    match todo_item {
        ItemTypes::Done(item) => {
            // println!("{}", item.super_struct.status.stringify());
            // println!("{}", item.super_struct.title);
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            // println!("{}", item.super_struct.status.stringify());
            // println!("{}", item.super_struct.title);
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title)
        }
    }
}
