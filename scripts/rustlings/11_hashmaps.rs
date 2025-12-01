use std::collections::HashMap;

// Exercise 1: Create a HashMap with three items and a sum of their values greater than 5.
fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 5);
    basket.insert(String::from("pear"), 3);
    basket
}

fn main() {
    // Exercise 1
    let basket = fruit_basket();
    dbg!(&basket);  // Note: the hashmap is unordered
    assert!(basket.len() == 3);
    assert!(basket.values().sum::<u32>() >= 5);

    println!("All tests passed!");
}
