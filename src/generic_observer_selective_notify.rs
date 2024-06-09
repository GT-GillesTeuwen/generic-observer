use generic_observer_macros::selective_notify;
use std::sync::{Arc, Mutex};

#[selective_notify(value,colour)]
struct Cell {
    name:String,
    value: i32,
    colour:String,
}


struct CellFanatic {
    num_updates:i32,
    total:i32
}

impl CellFanatic {
    pub fn increment_updates(&mut self){
        self.num_updates+=1;
    }
    pub fn add_to_total(&mut self,cell:&Cell){
        self.total+=cell.value;
    }
}


pub fn main() {
    let mut cell_a = Cell::new("a".to_string(),10,"\x1b[91m".to_string()); // make cell a

    let cell_b =Cell::new("b".to_string(),2*cell_a.value,"\x1b[92m".to_string()); // make cell b
    let cell_b_arc_mutex = Arc::new(Mutex::new(cell_b)); //put cell b in arc mutex for threading (notify requires send and sync)
    let cell_b_clone = Arc::clone(&cell_b_arc_mutex); // clone to give to the observer

    let c_f=CellFanatic{
        num_updates:0,
        total:0
     };
     let c_f_am=Arc::new(Mutex::new(c_f));
     let c_f_clone_1=Arc::clone(&c_f_am);
     let c_f_clone_2=Arc::clone(&c_f_am);

    cell_a.register_value_observer(Box::new(move |c| {
        println!("{}cell_{} is now {}\x1b[0m",c.colour,c.name,c.value);
    })); //register to print when a is changed

    cell_a.register_value_observer(Box::new(move |c| {
        let mut locked=c_f_clone_1.lock().unwrap();
        locked.increment_updates();
    })); //increment cell fanatic's number of updates

    cell_a.register_value_observer(Box::new(move |c| {
        let mut cell_b_locked = cell_b_clone.lock().unwrap();
        cell_b_locked.set_value(c.value * 2);
    })); //register observer to update b when a is changed

    cell_b_arc_mutex.lock().unwrap().register_value_observer(Box::new(move |c| {
        println!("{}cell_{} is now {}\x1b[0m",c.colour,c.name,c.value);
    })); //register to print when b is changed

    cell_b_arc_mutex.lock().unwrap().register_value_observer(Box::new(move |c| {
        let mut locked=c_f_clone_2.lock().unwrap();
        locked.add_to_total(c);
    })); // add to cell fanatics total



    // Update cell_a to check if cell_b gets changed
    cell_a.set_value(15);
    println!("\n");

    // Update cell_a to check if cell_b gets changed
    cell_a.set_value(2);
    println!("\n");

    // Update cell_a's colour to show that nothing fires
    cell_a.colour="\x1b[94m".to_string();

    // Update cell_a to check if cell_b gets changed
    cell_a.set_value(-100);
    println!("\n");

    println!("cell_b's final value: {}",cell_b_arc_mutex.lock().unwrap().value); //Proving that the changes are applied everywhere
    let locked=c_f_am.lock().unwrap();
    println!("The number of times a was updated is: {}\nThe total of all values in b is:{}",locked.num_updates,locked.total); 
}
