use std::{thread, time::Duration};

fn main() {
    let time = 10;
    let handle = thread::spawn(move || {
        for i in 0..time {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
