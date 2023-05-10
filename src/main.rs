mod todo;

// use todo::structs::done::Done;

// use todo::structs::pending::Pending;

use todo::enums::TaskStatus;
use todo::todo_factory;
use todo::ItemTypes;

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
            println!("{}", item.super_struct.status.stringify());
            println!("{}", item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            println!("{}", item.super_struct.status.stringify());
            println!("{}", item.super_struct.title);
        }
    }
}
