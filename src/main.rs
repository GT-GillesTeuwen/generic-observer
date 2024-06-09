mod initial_observer;
mod gof_observer;
mod generic_observer_selective_notify;
mod generic_observer_notify;

fn main(){
    initial_observer::main();

    gof_observer::main();

    generic_observer_notify::main();

    generic_observer_selective_notify::main();

}