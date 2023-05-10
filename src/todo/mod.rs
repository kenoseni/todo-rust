/// Logic that handles the management of todo items in our todo module
pub mod enums;
pub mod structs;

// Factory method
use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn todo_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
    }
}
