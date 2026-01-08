// Exercise 1: Create a function that returns a Result type depending on the input provided.
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
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

    println!("All tests passed!");
}
