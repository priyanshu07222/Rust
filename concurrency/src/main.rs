use std::thread;
use std::time::Duration;
fn main() {
    // Thread
    println!("Hello, world!");

    // let handle = thread::spawn(|| {
    //     for i in 1..11 {
    //         println!("hi number {i} from the spawned thread!");
    //         // thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..10 {
    //     println!("hi number {i} from the main thread!");
    //     // thread::sleep(Duration::from_millis(1));
    // }

    // handle.join();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();

    // Message Concurrency

    
}
