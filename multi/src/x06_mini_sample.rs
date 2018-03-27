use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::rc::Rc;

use std::thread::sleep_ms;
use std::sync::mpsc::Sender;

use std::sync::Mutex;

struct Listener {
}

impl Listener {
    fn job_done(&self, id : i32) {
        println!("job done, i am informed about {}", id);
    }
}

struct InformingWorker<'a> {
    listener : &'a Listener
}

impl<'a> InformingWorker<'a> {
    fn new(listener : &Listener) ->InformingWorker {
        InformingWorker {
            listener
        }
    }

    fn do_and_inform(&mut self, job : i32) {
        self.listener.job_done(job + 1);
    }
}

#[test]
fn mini_sample() {
    let listener = Listener {};
    let mut worker = InformingWorker::new(&listener);

    worker.do_and_inform(1);
}






























struct InformingWorkerAsync {
    listener : Arc<Mutex<Listener>>,
    sender : Sender<i32>,

    worker_thread : JoinHandle<()>,
}

impl InformingWorkerAsync {
    fn new(listener : Arc<Mutex<Listener>>) -> InformingWorkerAsync {

        let (tx, rx) = channel();
        let cloned = listener.clone();
        let worker_thread = thread::spawn(move ||{
            let mutex = cloned;

            for job in rx {
                mutex.lock().unwrap().job_done(job + 1);
            }
        });

        InformingWorkerAsync {
            listener,
            worker_thread,
            sender : tx,
        }
    }

    fn do_and_inform(&mut self, job : i32) {

        let listener = self.listener.clone();
        self.sender.send(job);
    }
}

#[test]
fn mini_sample_cheatsheet() {
    let listener = Listener {};
    let listener = Arc::new(Mutex::new(listener));
    let mut worker = InformingWorkerAsync::new(listener);

    worker.do_and_inform(1);

    sleep_ms(1000);
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
