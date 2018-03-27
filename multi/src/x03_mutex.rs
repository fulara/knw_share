use std::thread;
use std::sync::Arc;
use std::thread::sleep_ms;

use std::sync::Mutex;

#[test]
fn mutex_silly_sample() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}



#[test]
fn mutex_sharing_data() {
//    let v: Vec<i32> = Vec::new();
//
//    //finish me - write a program that modifies that vector.
//
//    loop {
//        {
//            let data: &Vec<i32> = unimplemented!();
//            println!("main thread is peeking at data! {:?}", data);
//        }
//
//        sleep_ms(1000);
//    }
}























#[test]
fn mutex_shared_multiple_threads() {
//    let m = Mutex::new(Vec::new());
//
//    for i in 0 .. 10 {
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


//#[test]
//fn mutex_shared_multiple_threads_cheatsheet() {
//    let m = Arc::new(Mutex::new(Vec::new()));
//
//    for i in 0 .. 10 {
//        let m = Arc::clone(&m); // equivalent to m.clone();
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
//}