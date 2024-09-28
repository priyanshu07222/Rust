use std::sync::{Arc, Mutex};
use std::thread;
// use std::time::Duration;
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

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {v:?}");
    // });

    // handle.join().unwrap();

    // Message Concurrency

    // let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {received}");
    // }

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        let x = handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
