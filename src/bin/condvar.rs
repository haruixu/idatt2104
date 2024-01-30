use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

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
}

enum Message {
    NewTask(Task),
    Terminate,
}

//Declare a type for the task to avoid syntax cluttering
type Task = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
#[allow(unused_variables)]
///
/// Arc allows shared variables (mutex. task queue, condvar) to be shared - drops with zero
/// references.
///
/// Declare a task type to avoid needing to specify type each time
///
/// Threads vec is needed to store the threads.
///
/// Is_finished flag is used to notify threads when to break loop
/// Saw that you maybe could you condvar - broadcast (but gpt suggestion).
struct Workers {
    nr_workers: usize,
    tasks: Arc<(Mutex<Vec<Task>>, Condvar)>,
    threads: Vec<thread::JoinHandle<()>>, //Need to store thread somewhere
    is_finished: bool,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Workers {
    fn new(nr_workers: usize) -> Self {
        //Må clone, men er løsningen å passe fra main?
        let tasks = Arc::new((Mutex::new(Vec::<Task>::new()), Condvar::new()));
        let threads = Vec::with_capacity(nr_workers);

        //how the fuck do i while() on a variable that has yet to be declared????

        for i in 0..nr_workers {
            let thread = thread::spawn(move || loop {
                let (lock, cvar) = &*tasks;

                let mut task_queue = lock.lock().unwrap();

                while task_queue.is_empty() {
                    task_queue = cvar.wait(task_queue).unwrap();
                }

                let task = task_queue.pop().unwrap();
                drop(lock);

                task();
            });
        }

        Workers {
            nr_workers,
            tasks,
            threads,
            is_finished: false,
        }
    }

    fn post(&self, task: Task) {
        let (lock, cvar) = &*self.tasks;

        let mut queue = lock.lock().unwrap();
        queue.push(task);

        cvar.notify_one();
    }

    fn stop(&mut self) {
        self.is_finished = true;
        let (lock, cv) = &*self.tasks;
        let mut queue = lock.lock().unwrap();
        queue.clear();

        cv.notify_all();
    }

    fn post_timeout(&self, task: Task, delay: u64) {
        thread::sleep(Duration::from_millis(delay));
        Workers::post(self, task);
    }

    //Impl Drop for Workers???
    fn drop(&mut self) {
        Workers::stop(self);

        for thread in &self.threads {
            let _ = &thread.join();
        }
    }
}
