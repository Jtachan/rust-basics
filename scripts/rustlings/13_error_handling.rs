use std::num::{IntErrorKind, ParseIntError};

// Exercise 1: Create a function that returns a Result type depending on the input provided.
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

/*
   Exercise 2:
       Asume a program where the user inputs a quantity (as a string) and the total cost to
       buy that quantity is calculated by the function 'total_cost'.

       Code a routine to return a `ParseIntError` when the input string does not contain only
       numbers. No math operation must be performed in case of any error.
       There is two ways to solve this exercise.
*/
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let process_fee = 1;
    let cost_per_item = 5;

    // The `?` operator works as a 'match' returning the Err if there is any.
    // The following line is analogous to the commented code.
    let qty = item_quantity.parse::<i32>()?;
    // let qty = match item_quantity.parse::<i32>() {
    //     Ok(v) => v,
    //     Err(e) => return Err(e),
    // };

    Ok(qty * cost_per_item + process_fee)
}

fn main() {
    // Exercise 1
    assert_eq!(
        generate_nametag_text("Beyoncé".to_string()).as_deref(),
        Ok("Hi! My name is Beyoncé"),
    );
    assert_eq!(
        generate_nametag_text(String::new())
            .as_ref()
            .map_err(|e| e.as_str()),
        Err("Empty names aren't allowed"),
    );

    // Exercise 2
    assert_eq!(total_cost("34"), Ok(171));
    assert_eq!(
        total_cost("beep boop").unwrap_err().kind(),
        &IntErrorKind::InvalidDigit
    );

    println!("All tests passed!");
}
