use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    // Deadlock
    let m1 = Arc::new(Mutex::new(0));
    let m2 = Arc::new(Mutex::new(0));

    let m1_clone = Arc::clone(&m1);
    let m2_clone = Arc::clone(&m2);
    let v1 = m1.lock().unwrap();

    thread::spawn(move || {
        let v2 = m2_clone.lock().unwrap();
        let v1 = m1_clone.lock().unwrap();
    });

    thread::sleep(Duration::from_millis(2));
    let v2 = m2.lock().unwrap();


    // MutExes and Atomic Reference Counting (Arc)
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
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Channels - Multiple Producers, Single Consumer
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // Spawning and joining Threads
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
