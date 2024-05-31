use generic_observer_macros::notify;
use std::sync::{Arc, Mutex};

#[notify]
struct Cell {
    name:String,
    value: i32,
}

fn main() {
    let mut cell_a = Cell::new("a".to_string(),10);

    let cell_b = Arc::new(Mutex::new(Cell::new("b".to_string(),2*cell_a.value)));
    let cell_b_clone = Arc::clone(&cell_b);

    cell_a.register_value_observer(Box::new(move |c| {
        
        let mut cell_b_locked = cell_b_clone.lock().unwrap();
        cell_b_locked.value = c.value * 2;
        println!("Cell a is {} so updated cell b to {}",c.value,cell_b_locked.value);
    }));

    // Update cell_a
    cell_a.set_value(15);

    cell_a.set_value(2);
}
