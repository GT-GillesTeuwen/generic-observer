mod initial_observer;
mod gof_observer;
mod generic_observer_selective_notify;
mod generic_observer_notify;

fn main(){
    println!("\n#####################################################");
    println!("Initial observer implementation\n");
    initial_observer::main();
    println!("\n#####################################################");
    println!("Emulating OOP observer implementation\n");
    gof_observer::main();
    println!("\n#####################################################");
    println!("Initial generic observer implementation\n");
    generic_observer_notify::main();
    println!("\n#####################################################");
    println!("Selective Notify generic observer implementation\n");
    generic_observer_selective_notify::main();

}