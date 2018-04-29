use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread::{spawn, sleep};

#[test]
fn channels() {
    let (tx, rx) = channel();

    let handle = spawn(move || for m in rx.iter() {
                           println!("got .. {:?}", m);
                       });

    for i in 0..10 {
        sleep(Duration::from_millis(500));

        tx.send(i);
    }
}

#[test]
fn multiple_producers() {
    let (tx, rx) = channel();

    let handles = (0..3)
        .into_iter()
        .map(|i| {
            let tx = tx.clone();
            spawn(move || for counter in 0.. {
                      tx.send(i * 1000 + counter);
                      sleep(Duration::from_millis(1000));
                  })
        })
        .collect::<Vec<_>>();

    for x in rx {
        println!("received : {}", x);
    }
}
