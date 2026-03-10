use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Exercise 1: spawning threads
fn exercise_1() {
    let exercise_start = Instant::now();

    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} is done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    assert_eq!(results.len(), 10);
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result} ms");
    }

    println!(
        "The program took {} ms",
        exercise_start.elapsed().as_millis()
    );
}

// Exercise Arc pointers
fn atomic_rec_counter() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut join_handles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}

fn main() {
    println!("\nRunning exercise 1...");
    exercise_1();

    println!("\nRunning Arc exercise...");
    atomic_rec_counter();

    println!("\nAll exercises passed!");
}
