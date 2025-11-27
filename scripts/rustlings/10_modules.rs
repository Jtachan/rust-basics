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

fn main() {
    println!("\nExercise 1:");
    sausage_factory::make_sausage();

    println!("\nExercise 2:");
    println!("favourite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    )
}
