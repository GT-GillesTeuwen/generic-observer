use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

// Define the Observer trait
trait Observer {
    fn update(&self, value: i32, colour: &str, name: &str, id: usize);
}

// A structure to hold Observer references
#[derive(Clone)]
struct ObserverHandle {
    id: usize,
    callback: Arc<dyn Observer + Send + Sync>,
}

impl Hash for ObserverHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for ObserverHandle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ObserverHandle {}

// Define the Observable structure
struct Cell {
    name: String,
    value: i32,
    colour: String,
    observers: Mutex<HashSet<ObserverHandle>>,
}

impl Cell {
    // Constructor for Cell
    fn new(name: String, value: i32, colour: String) -> Self {
        Cell {
            name,
            value,
            colour,
            observers: Mutex::new(HashSet::new()),
        }
    }

    // Method to add an observer
    fn add_observer(&self, observer: Arc<dyn Observer + Send + Sync>, id: usize) {
        let observer_handle = ObserverHandle { id, callback: observer };
        self.observers.lock().unwrap().insert(observer_handle);
    }

    // Method to remove an observer
    fn remove_observer(&self, id: usize) {
        let temp_handle = ObserverHandle { id, callback: Arc::new(DummyObserver {}) };  // Dummy observer for removal
        self.observers.lock().unwrap().remove(&temp_handle);
    }

    // Set the value and notify observers
    fn set_value(&mut self, value: i32) {
        self.value = value;
        self.notify();
    }

    // Notify all observers about the change
    fn notify(&self) {
        let observers = self.observers.lock().unwrap();
        for observer in observers.iter() {
            observer.callback.update(self.value, &self.colour, &self.name, observer.id);
        }
    }
}

// Dummy Observer for removal
struct DummyObserver;

impl Observer for DummyObserver {
    fn update(&self, _value: i32, _colour: &str, _name: &str, _id: usize) {
        // No action needed
    }
}

// An example observer
struct PrintObserver;

impl Observer for PrintObserver {
    fn update(&self, value: i32, colour: &str, name: &str, id: usize) {
        println!("{}cell_{} value changed to {} - Observer ID:{}\x1b[0m", colour, name, value, id);
    }
}

fn main() {
    let cell_a = Arc::new(Mutex::new(Cell::new("a".to_string(), 10, "\x1b[91m".to_string())));
    let observer = Arc::new(PrintObserver {});

    // Clone the observer before passing it to add_observer
    cell_a.lock().unwrap().add_observer(observer.clone(), 1);

    cell_a.lock().unwrap().set_value(15);
    cell_a.lock().unwrap().set_value(2);

    // Remove observer ID 1
    cell_a.lock().unwrap().remove_observer(1);

    // Re-add observer with a different ID
    cell_a.lock().unwrap().add_observer(observer.clone(), 2);

    cell_a.lock().unwrap().set_value(-100);

    // Add the same observer with another ID
    cell_a.lock().unwrap().add_observer(observer.clone(), 3);
    
    cell_a.lock().unwrap().set_value(100);
}

