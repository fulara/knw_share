use std::thread;
use std::thread::sleep_ms;
use std::sync::mpsc::channel;


#[derive(Debug)]
struct SomeData {
    field : i32,
}

#[test]
fn channels() {
    let (tx, rx) = channel();

    for x in 0 .. 2 {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut i = 0;
            loop {
                sleep_ms(500);

                tx.send(SomeData { field: x* 1000 + i });

                i += 1;
            }
        });
    }

    for data in rx {
        println!("got data {:?}", data);
    }
}
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//#[derive(Debug)]
//struct Data {
//    f : i32,
//}
//
//#[test]
//fn channels_cheatsheet() {
//    let (tx, rx) = channel();
//
//    let handle = thread::spawn(move|| {
//        let mut x = 0;
//        loop {
//            tx.send(Data { f : x});
//            x += 1;
//
//            sleep_ms(1000);
//        }
//    });
//
//    for d in rx {
//        println!("hai from main thread...! {:?}", d);
//    }
//}
