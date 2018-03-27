



extern crate crossbeam;

use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::rc::Rc;

use std::thread::sleep_ms;
use std::sync::mpsc::Sender;

use std::sync::Mutex;


#[test]
fn crossbeam_example() {
    let array = [1, 2, 3];

    crossbeam::scope(|scope| {
        for i in &array {
            scope.spawn(move || {
                println!("element: {}", i);
            });
        }
    });
}