use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub trait Observer<T> {
    fn update(&mut self, event: &T);
}


#[derive(Default)]
pub struct ObserverManager<T> {
    observers: HashMap<usize, Arc<Mutex<dyn Observer<T> + Send + Sync>>>,
    next_id: usize,
}

impl<T: 'static> ObserverManager<T> {
    pub fn notify(&self, event: &T) {
        for observer in self.observers.values() {
            observer.lock().unwrap().update(event);
            
        }
    }

    pub fn add_observer(&mut self, observer: Arc<Mutex<dyn Observer<T> + Send + Sync>>) -> usize {
        let id = self.next_id;
        self.observers.insert(id, observer );
        self.next_id += 1;
        id
    }

    pub fn remove_observer(&mut self, id: usize) {
        self.observers.remove(&id);
    }
}

pub trait Subject<T> {
    fn notify(&self, event: &T);

    fn add_observer(&mut self, observer: Arc<Mutex<dyn Observer<T> + Send + Sync>>) -> usize ;

    fn remove_observer(&mut self, id: usize);
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
    pub observer_manager: ObserverManager<CellEvent>,
}

impl Cell {
    pub fn set_value(&mut self,val:i32) {
        self.value=val;
        println!("cell_{} value is now {}",self.name,self.value);
        self.observer_manager.notify(&CellEvent::Updated(self.value));
    }

}

impl Subject<CellEvent> for Cell {
    fn notify(&self, event: &CellEvent) {
        self.observer_manager.notify(event);
    }

    fn add_observer(&mut self, observer: Arc<Mutex<dyn Observer<CellEvent> + Send + Sync>>) -> usize {
        self.observer_manager.add_observer(observer)
    }

    fn remove_observer(&mut self, id: usize) {
        self.observer_manager.remove_observer(id)
    }
}


impl Observer<CellEvent> for Cell {
    fn update(&mut self, event: &CellEvent) {
        match event {
            CellEvent::Updated(val) => self.set_value(2*val),
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

impl Observer<CellEvent> for CellFanatic{
    fn update(&mut self, event: &CellEvent) {
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
    let id1 = b.lock().unwrap().add_observer(clone);
    let id2 = a.add_observer(b);
   
    let id3 = a.add_observer(cell_fanatic);
   
    a.set_value(2);

    a.remove_observer(id3);
    println!("\n");
    a.set_value(10);
}