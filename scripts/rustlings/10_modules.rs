/*  Exercise 1: Correct use of 'pub' only where it is needed */
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

/* Exercise 2: make use of the keywords 'use' and 'as' */
mod delicious_snacks {
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    // Unused items have been commented to avoid compiler warnings
    mod fruits {
        pub const PEAR: &str = "Pear";
        // pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        // pub const CARROT: &str = "Carrot";
    }
}

/*
   Exercise 3: Use of standard libraries into scope.
   Import structure 'SystemTime' and constant 'UNIX_EPOCH' from 'std::time'.
   This can be performed in a single line.
*/
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("\nExercise 1:");
    sausage_factory::make_sausage();

    println!("\nExercise 2:");
    println!(
        "favourite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );

    println!("\nExercise 3:");
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH! (what did you do to reach here?)"),
    }

    println!("All test passed!");
}
