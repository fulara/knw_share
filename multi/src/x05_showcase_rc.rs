use std::thread;
use std::sync::Arc;
use std::rc::Rc;
use std::thread::sleep_ms;

use std::sync::Mutex;


#[test]
fn rc_showcase() {
//    let m = Rc::new(Mutex::new(Vec::new()));
//
//    for i in 0 .. 10 {
//        let m = m.clone();
//        thread::spawn(move || {
//            let mut data = m.lock().unwrap();
//            println!("thread {} woke up and doing extensive work.", i);
//            sleep_ms(500);
//            data.push(i);
//        });
//    }
//
//    loop {
//        {
//            let data : &Vec<i32> = &m.lock().unwrap();
//            println!("main thread is peeking at data! {:?}", data);
//        }
//
//        sleep_ms(1000);
//    }
}





























#[test]
fn channels_cheatsheet() {
//    let (tx, rx) = channel();
//
//    let handle = thread::spawn(move|| {
//        loop {
//            tx.send("hai hai");
//
//            sleep_ms(1000);
//        }
//    });
//
//    for d in rx {
//        println!("hai from main thread...!");
//    }
}
