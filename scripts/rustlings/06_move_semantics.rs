fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // Shadowing 'vec' with a new 'vec' which is also mutable.
    let mut vec = vec;
    vec.push(88);
    vec
}

fn fill_vec_mut(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

// This function shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// This function should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{data}");
}

fn main() {
    // Exercise 1: Borrow
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0);
    assert_eq!(vec1, vec![22, 44, 66, 88]);

    // Exercise 2: Make both vectors `vec0` and `vec1` accessible at the same time.
    //  Do not modify the function.
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0.clone());
    assert_eq!(vec0, [22, 44, 66]);
    assert_eq!(vec1, [22, 44, 66, 88]);

    // Exercise 3: Fix 'fill_vec_mut' without modifying its body.
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec_mut(vec0);
    assert_eq!(vec1, [22, 44, 66, 88]);

    // Exercise 4: Fixing compiler errors by rearranging:
    // An object can have one borrow at the time. Each borrow must be used before a new one is called.
    let mut x = Vec::new();
    let y = &mut x;
    y.push(42);
    let z = &mut x;
    z.push(13);
    assert_eq!(x, [42, 13]);

    // Exercise 5: Ownership in functions
    let data = "Rust is great!".to_string();
    get_char(&data);
    string_uppercase(data);

    println!("All tests passed!")
}
