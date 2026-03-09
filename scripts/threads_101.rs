//! Here are some scripts taken directly out of the 'Concurrency' chapter (16.1) at the Rust Book.
//! The goal is just to have a very basic understanding of threads.
use std::thread;
use std::time::Duration;

fn spawn_threads(wait_for_threads: bool) {
    // Spawning sub-threads:
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Sleeping the main thread for 1 ms four times:
    for i in 1..5 {
        println!("Main thread call {i}!");
        thread::sleep(Duration::from_millis(1));
    }

    if wait_for_threads {
        // The following line blocks the code until all threads are done.
        handle.join().unwrap();
    }

    print!("Finished 'Spawning threads' function.");
}

fn main() {
    // Spawning and waiting threads:
    spawn_threads(true);


}
