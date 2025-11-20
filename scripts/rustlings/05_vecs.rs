// https://github.com/rust-lang/rustlings/tree/main/exercises/05_vecs

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    // Todo: Create a vector 'v' which contains the same elements an array 'a'.
    let a = [10, 20, 30, 40]; // Array
    let v = vec![10, 20, 30, 40];
    (a, v)
}

fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    // Todo: Multiply each element in the input slice by 2 and push it to the output vector.
    for element in input {

    }

    output
}

fn main() {
    let (a, v) = array_and_vec();
    assert_eq!(a, *v);

    println!("All exercises passed!")
}
