use std::{mem, sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    for _i in 0..10 {
        let tx = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            for i in 0..5 {
                tx.send(format!(
                    "thread: [id: {:?}, name: {:?}], {}",
                    thread::current().id(),
                    thread::current().name(),
                    i
                ))
                .unwrap();
                thread::sleep(Duration::from_millis(300));
            }
        });

        thread::sleep(Duration::from_millis(123));
    }

    // 要释放原 Sender，否则将一直等待无法退出
    mem::drop(tx);

    for received in rx {
        println!("Got: {}", received);
    }
    println!("main end...");
}
