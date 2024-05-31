use generic_observer_macros::notify;

// Define the macro and other necessary setups in scope
#[notify]
struct Character {
    health: i32,
    mana: i32,
    experience: i32,
}

fn main() {
    let mut hero = Character::new(100, 50, 0); // Initial stats: health, mana, experience

    register_all_observers(&mut hero);

    // Simulate a battle scenario
    hero.set_health(hero.health - 30); // Hero takes damage
    hero.set_mana(hero.mana - 20); // Hero casts a spell
    hero.set_experience(hero.experience + 50); // Hero gains some experience

    // Simulate a battle scenario
    hero.set_health(hero.health - 30); // Hero takes damage
    hero.set_mana(hero.mana - 20); // Hero casts a spell
    hero.set_experience(hero.experience + 50); // Hero gains some experience

    hero.set_health(0);
}

fn register_all_observers(hero: &mut Character) {
    // Register an observer for the health field
    hero.register_health_observer(Box::new(|c| {
        if c.health <= 0 {
            println!("{} has died.", "Hero"); // Customizing the message
        } else {
            println!("Hero's health is now: {}", c.health);
        }
    }));

    // Register an observer for the mana field
    hero.register_mana_observer(Box::new(|c| {
        println!("Hero's mana is now: {}", c.mana);
    }));

    // Register an observer for the experience field
    hero.register_experience_observer(Box::new(|c| {
        println!("Hero gained experience, now has: {}", c.experience);
        if c.experience >= 100 {
            println!("Hero has leveled up!");
        }
    }));
}
