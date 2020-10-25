use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

pub fn shared_memory() {
    let results = Arc::new(Mutex::new(Vec::<String>::new()));
    let mut handlers = Vec::new();

    for i in 0..5 {
        let results = Arc::clone(&results);
        let handler = thread::spawn(move || {
            for j in 0..100 {
                let mut unlocked_results = results.lock().unwrap();
                let result = format!("Completed Job {} on Thread {}", j, i);
                unlocked_results.push(result);
            }
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }
    for result in results.lock().unwrap().iter() {
        println!("{}", result);
    }
    println!("Finished running jobs with shared memory");

}

pub fn message_passing() {
    let (transmitter, receiver) = mpsc::channel();
    let mut results = Vec::<String>::new();
    let mut handlers = Vec::new();

    for i in 0..4 {
        let transmitter = transmitter.clone();
        let handler = thread::spawn(move || {
            for j in 0..100 {
                let result = format!("Completed Job {} on Thread {}", j, i);
                transmitter.send(result).unwrap();
            }
        });
        handlers.push(handler);
    }
    thread::spawn(move || {
        for j in 0..100 {
            let result = format!("Completed Job {} on main thread", j);
            transmitter.send(result).unwrap();
        }
    });
    for received in receiver {
        results.push(received);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
    for result in results {
        println!("{}", result);
    }
    println!("Finished running jobs with message passing");
}