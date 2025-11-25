// https://github.com/rust-lang/rustlings/tree/main/exercises/05_vecs

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    // Create a vector 'v' which contains the same elements an array 'a'.
    let a = [10, 20, 30, 40]; // Array
    let v = vec![10, 20, 30, 40];
    (a, v)
}

fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    // Multiply each element in the input slice by 2 and push it to the output vector.
    for element in input {
        output.push(element * 2)
    }

    output
}

fn main() {
    // Exercise 1: create vector from macro
    let (a, v) = array_and_vec();
    assert_eq!(a, *v);

    // Exercise 2: new vector from array
    let input = [2, 4, 6, 8, 10];
    let ans = vec_loop(&input);
    assert_eq!(ans, [4, 8, 12, 16, 20]);

    println!("All exercises passed!")
}
