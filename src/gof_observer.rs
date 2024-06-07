use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub trait Observe<T> {
    fn observe(&mut self, event: &T);
}

// Observer with an ID
pub struct ObserverHandle<T> {
    id: usize,
    observer: Arc<Mutex<dyn Observe<T> + Send + Sync>>,
}

#[derive(Default)]
pub struct Observers<T> {
    observers: HashMap<usize, ObserverHandle<T>>,
    next_id: usize,
}

impl<T: 'static> Observers<T> {
    pub fn notify(&self, event: &T) {
        for observer_handle in self.observers.values() {
            if let Ok(mut observer) = observer_handle.observer.lock() {
                observer.observe(event);
            }
        }
    }

    pub fn add_observer(&mut self, observer: Arc<Mutex<dyn Observe<T> + Send + Sync>>) -> usize {
        let id = self.next_id;
        self.observers.insert(id, ObserverHandle { id, observer });
        self.next_id += 1;
        id
    }

    pub fn remove_observer(&mut self, id: usize) {
        self.observers.remove(&id);
    }
}


// Usage
pub enum CellEvent {
    Updated(i32),
}

impl Default for CellEvent {
    fn default() -> Self {
        Self::Updated(0)
    }
}

#[derive(Default)]
pub struct Cell {
    pub name: String,
    pub value:i32,
    pub observers: Observers<CellEvent>,
}

impl Cell {
    pub fn update(&mut self,val:i32) {
        self.value=val;
        println!("cell_{} value is now {}",self.name,self.value);
        self.observers.notify(&CellEvent::Updated(self.value));
    }

}


impl Observe<CellEvent> for Cell {
    fn observe(&mut self, event: &CellEvent) {
        match event {
            CellEvent::Updated(val) => self.update(2*val),
        }
    }
}
#[derive(Default)]
struct CellFanatic {
    num_updates:i32,
    total:i32
}

impl CellFanatic {
    pub fn increment_updates(&mut self){
        self.num_updates+=1;
        println!("number of updates {}",self.num_updates);
    }
    pub fn add_to_total(&mut self,val:&i32){
        self.total+=val;
        println!("total of all values {}",self.total);
    }
}

impl Observe<CellEvent> for CellFanatic{
    fn observe(&mut self, event: &CellEvent) {
        match event {
            CellEvent::Updated(val) => {self.increment_updates(); self.add_to_total(val)}
        }
    }
}

pub fn main() {

    let cell_fanatic=Arc::new(Mutex::new(CellFanatic::default()));
    let clone=Arc::clone(&cell_fanatic);

    let mut a = Cell::default();
    a.name="a".to_string();
    let b =  Arc::new(Mutex::new(Cell::default()));
    b.lock().unwrap().name="b".to_string();
    let id1 = b.lock().unwrap().observers.add_observer(clone);
    let id2 = a.observers.add_observer(b);
   
    let id3 = a.observers.add_observer(cell_fanatic);
   
    a.update(2);

    a.observers.remove_observer(id1);
    println!("\n");
    a.update(10);
}