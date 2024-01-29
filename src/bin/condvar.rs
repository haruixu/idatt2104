use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let worker_threads = Workers::new(4);
    let event_loop = Workers::new(1);

    //Creating a pair each for worker threads and event loops to prevent contention for same lock
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_a = Arc::clone(&pair);
    let pair_b = Arc::clone(&pair);

    let pair2 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2_a = Arc::clone(&pair2);
    let pair_b = Arc::clone(&pair2);

    /*worker_threads.start(); //Create 4 internal threads
    event_loop.start(); //Create 2 interanl thread

    worker_threads.post(|| {});
    worker_threads.post(|| {});

    event_loop.post(|| {});
    event_loop.post(|| {});

    worker_threads.join();
    event_loop.join();*/

    /*todo initiere threads som sitter og venter på at tasks skal dukke opp (start)
     * Legge til tasks i task-listen
     * Poppe av tasklisten som skal være protected med lock
     *      Bruke condvar for om task-listen er ledig
     *      Kjør tasken utenfor loopen2
     * Legge til stop() som automatisk breaker løkka (breaker while true)
     * Legge til post_timeout som venter x-sec før tasken kjøres*/
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct Workers {
    nr_workers: usize,
    tasks: Vec<fn()>,
    //threads: Vec<JoinHandle<()>>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Workers {
    fn new(nr_workers: usize) -> Self {
        Workers {
            nr_workers,
            tasks: Vec::new(),
        }
    }

    fn start(&self, pair: Arc<(Mutex<bool>, Condvar)>) {
        let handler = thread::spawn(move || {});

        //todo instantiate x-threads
        //thread spawn and such

        /*thread::spawn(move|| {
                    let (lock, cvar) = &*pair2;
                    let mut started = lock.lock().unwrap();
                    *started = true;
                    // We notify the condvar that the value has changed.
                    cvar.notify_one();
        });*/
    }

    fn post(&mut self, task: fn()) {
        //todo pass task into vector
        //possibly needing a mutex?
        self.tasks.push(task);
    }

    fn stop(&self) {
        //todo stop when vec is empty (call it inside post?)
        //probs need mutex because you need to pop off item if not empty??
    }

    fn post_timeout(&self, delay: usize) {
        //todo start running function after x millisec
    }

    fn join(&self) {
        //for each thread in threads, join();
    }
}
