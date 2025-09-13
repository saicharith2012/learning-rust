use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..100 {
            println!("the number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..100 {
        println!("the number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1,2,3];

    // ownership of 'v' moves to the thread.
    thread::spawn(move || {
        println!("{:?}", v);
    });
}
