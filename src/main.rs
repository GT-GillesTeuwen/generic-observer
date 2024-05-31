use generic_observer_macros::notify;
use std::sync::{Arc, Mutex};

#[notify]
struct Cell {
    name:String,
    value: i32,
}

fn main() {
    let mut cell_a = Cell::new("a".to_string(),10); // make cell a
    let cell_b =Cell::new("b".to_string(),2*cell_a.value); // make cell b

    let cell_b_arc_mutex = Arc::new(Mutex::new(cell_b)); //put cell b in arc mutex for threading (notify requires send and sync)
    let cell_b_clone = Arc::clone(&cell_b_arc_mutex); // clone to give to the observer

    cell_a.register_value_observer(Box::new(move |c| {
        let mut cell_b_locked = cell_b_clone.lock().unwrap();
        cell_b_locked.value = c.value * 2;
        println!("Cell {} is {} so updated cell {} to {}",c.name,c.value,cell_b_locked.name, cell_b_locked.value);
    })); //register observer with functionality to fire when a change is detected

    // Update cell_a to check if cell b gets changed
    cell_a.set_value(15);

    cell_a.set_value(2);
}
