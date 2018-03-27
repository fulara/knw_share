use std::thread;

#[test]
fn simple_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

//    println!("Here's a vector: {:?}", v);

    handle.join();
}
