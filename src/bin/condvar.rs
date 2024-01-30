use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut worker_threads = Workers::new(4); //Create 4 internal threads
    let mut event_loop = Workers::new(1); //Create 1 internal thread

    worker_threads.post(Box::new(|| {
        let mut sum = 0;
        for i in 0..10 {
            sum += i;
        }
        println!("Sum: {sum}");
    }));

    /*worker_threads.post(Box::new(|| {
        let mut sum = 0;
        for i in 0..20 {
            sum += i;
        }
        println!("Sum: {sum}");
    }));

    event_loop.post(Box::new(|| {
        let mut sum = 0;
        for i in 0..30 {
            sum += i;
        }
        println!("Sum: {sum}");
    }));

    event_loop.post(Box::new(|| {
        let mut sum = 0;
        for i in 0..40 {
            sum += i;
        }
        println!("Sum: {sum}");
    }));*/

    worker_threads.join();

    println!("falling out of scope");
}

//Declare a type for the task to avoid syntax cluttering
type Task = Box<dyn FnOnce() + Send + 'static>;

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
    tasks: Arc<(Mutex<Vec<Task>>, Condvar)>,
    threads: Vec<thread::JoinHandle<()>>, //Need to store thread somewhere
    is_finished: bool,
}

impl Workers {
    fn new(nr_workers: usize) -> Self {
        let tasks = Arc::new((Mutex::new(Vec::<Task>::new()), Condvar::new()));
        let mut threads = Vec::with_capacity(nr_workers);
        let is_finished = false;

        for _ in 0..nr_workers {
            let tasks = Arc::clone(&tasks);

            let thread = thread::spawn(move || {
                println!("Creating thread");
                while !is_finished {
                    let (lock, cvar) = &*tasks;

                    let mut task_queue = lock.lock().unwrap();

                    while task_queue.is_empty() {
                        println!("Waiting");
                        task_queue = cvar.wait(task_queue).unwrap();
                    }

                    println!("Finished waiting");

                    //Assume none-option never occurs
                    let task = task_queue.pop().unwrap();

                    //Releasing lock before running task
                    drop(lock);
                    task();
                }
            });
            println!("pushing thread to threads");
            threads.push(thread);
        }

        Workers {
            tasks,
            threads,
            is_finished,
        }
    }

    fn post(&self, task: Task) {
        println!("Creating task");
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
        println!("Just cleared tasks");

        cv.notify_all();
    }

    fn post_timeout(&self, task: Task, delay: u64) {
        thread::sleep(Duration::from_millis(delay));
        Workers::post(self, task);
    }

    fn join(&mut self) {
        //While is_finished = false - vent på en cv for is_finished som notifyer når vecen er tom i
        //thread metoden?????
        println!("Calling jion");
        Workers::stop(self);

        println!("isfinished: {}", self.is_finished);
        let queue = std::mem::take(&mut self.threads);
        for thread in queue {
            let _ = thread.join().unwrap();
        }
    }
}

//Not really doing anything here, because dropping is essentially handled by join, but nice
//having tried to implement at least once
impl Drop for Workers {
    fn drop(&mut self) {
        println!("dropping + is_finished={}", self.is_finished);
        Workers::stop(self);

        let queue = std::mem::take(&mut self.threads);
        for thread in queue {
            let _ = thread.join();
        }
    }
}
