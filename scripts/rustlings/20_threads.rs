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

fn main() {
    exercise_1();
}
