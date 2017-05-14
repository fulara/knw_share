

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Foo {
    v: i32,
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::rc::Rc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time;
    use std::sync::mpsc;
    use super::Foo;
    #[test]
    fn spawning_thread() {
        let handle = thread::spawn(|| {
                                       println!("hello from different thread");
                                   });

        handle.join();
        println!("hello from main thread");
    }

    #[test]
    fn rc_cannot_be_transferred() {
        let foo = Rc::new(Foo { v: 1 });

        /*let handle = {
            let foo = foo.clone();
            thread::spawn(move || {
                println!("foo {:?}", foo);
            })
        };
        handle.join(); */
    }

    #[test]
    fn arc_mutex() {
        let foo = Arc::new(Mutex::new(Foo { v: 1 }));

        let foo_clone = foo.clone();
        let handle = thread::spawn(move || {
                                       thread::sleep(time::Duration::from_millis(100));
                                       foo_clone.lock().unwrap().v += 1;
                                       println!("hai THREAD1 {:?}", foo_clone);
                                   });

        println!("hai MAIN 1{:?}", foo);
        handle.join();
        println!("hai MAIN 2 {:?}", foo);
    }

    fn produce(c: i32) -> Foo {
        thread::sleep(time::Duration::from_millis(1000));
        println!("producer producing {}", c);
        Foo { v: c }
    }

    fn consume(product: Foo) {
        println!("consumer consumed {:?}", product);
    }

    #[test]
    fn channels() {
        let (tx, rx) = mpsc::channel();

        {
            tx.clone();
            thread::spawn(move || {
                let mut c = 1;
                loop {
                    tx.send(produce(c));
                    c += 1
                }
            });
        }

        loop {
            consume(rx.recv().unwrap());
        }
    }


    #[test]
    fn atomics() {
        let atomic = Arc::new(AtomicBool::new(false));

        let handle = {
            let atomic = atomic.clone();
            thread::spawn(move || { atomic.store(true, Ordering::SeqCst); })
        };

        handle.join();
        println!("atomic before: {:?}", atomic);
        atomic.store(false, Ordering::SeqCst);
        println!("atomic after: {:?}", atomic);

    }
}
