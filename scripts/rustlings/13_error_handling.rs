use std::cmp::Ordering;
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

// Exercise 3: Fix the compiler by changing the signature and body of the `buy_with_tokens` function.
fn buy_with_tokens() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?; // This line must not be changed

    if cost > tokens {
        println!("You can't afford that many");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }

    // The type '()' is the Unit Type. Other languages use `null` or `void` instead.
    Ok(())
}

/*
   Exercise 4:
*/
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // This function shouldn't always return an Ok

        /*
            Solution 1: Use an if-else block
            --------------------------------
            if value < 0 {
                Err(CreationError::Negative)
            } else if value == 0 {
                Err(CreationError::Zero)
            } else {
                Ok(Self(value as u64))
            }
        */

        // Solution 2: Use match compare
        match value.cmp(&0) {
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Greater => Ok(Self(value as u64)),
        }
    }
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

    // Exercise 3
    let _ = buy_with_tokens();

    // Exercise 4
    assert_eq!(
        PositiveNonzeroInteger::new(10),
        Ok(PositiveNonzeroInteger(10))
    );
    assert_eq!(
        PositiveNonzeroInteger::new(-10),
        Err(CreationError::Negative)
    );
    assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));

    println!("All tests passed!");
}
