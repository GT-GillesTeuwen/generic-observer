use generic_observer_macros::notify;
use std::{borrow::Borrow, sync::{Arc, Mutex}};

#[notify]
struct Cell {
    name:String,
    value: i32,
}

fn main() {
    let mut cell_a = Cell::new("a".to_string(),10); // make cell a
    let cell_b =Cell::new("b".to_string(),2*cell_a.value); // make cell b

    let cell_b = Arc::new(Mutex::new(cell_b)); //put cell b in arc mutex for threading (notify requires send and sync)
    let cell_b_clone = Arc::clone(&cell_b); // clone to give to the observer

    cell_a.register_value_observer(Box::new(move |c| {
        println!("cell_{} is now {}",c.name,c.value);
    })); //register to print when a is changed

    cell_a.register_value_observer(Box::new(move |c| {
        let mut cell_b_locked = cell_b_clone.lock().unwrap();
        cell_b_locked.set_value(c.value * 2);
    })); //register observer to update b when a is changed

    cell_b.lock().unwrap().register_value_observer(Box::new(move |c| {
        println!("cell_{} is now {}",c.name,c.value);
    })); //register to print when b is changed

    // Update cell_a to check if cell_b gets changed
    cell_a.set_value(15);
    println!("\n");
    // Update cell_a to check if cell_b gets changed
    cell_a.set_value(2);
    println!("\n");
    // Update cell_a to check if cell_b gets changed
    cell_a.set_value(-100);
    println!("\n");

    println!("cell_b's final value: {}",cell_b.lock().unwrap().value); //Proving that the changes are applied everywhere
}
