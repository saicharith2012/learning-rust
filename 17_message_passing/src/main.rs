use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {received} ");

    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in (i * 10_000_000)..((i + 1) * 10_000_000) {
                ans += j;
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx); // important so rx knows when to stop

    let mut sum: u64 = 0;
    for val in rx {
        sum += val;
        println!("found value");
    }

    println!("Ans is {}", sum);
}
