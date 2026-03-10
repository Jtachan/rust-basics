macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

mod macros {
    #[macro_export]
    macro_rules! my_macro_2 {
        () => {
            println!("Check out my other macro!");
        };
    }
}

#[rustfmt::skip]
macro_rules! my_new_macro {
    () => {
        println!("Check out my new macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro_2!();
    my_new_macro!();
    my_new_macro!(777);
}
