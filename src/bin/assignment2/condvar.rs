use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut worker_threads = Workers::new(4); //Create 4 internal threads
    let mut event_loop = Workers::new(1); //Create 1 internal thread

    worker_threads.post(Message::NewTask(Box::new(|| {
        let mut sum = 0;
        for i in 0..10 {
            sum += i;
        }
        println!("Task 1 done: {sum}");
    })));

    worker_threads.post(Message::NewTask(Box::new(|| {
        let mut sum = 0;
        for i in 0..20 {
            sum += i;
        }
        println!("Task 2 done: {sum}");
    })));

    event_loop.post(Message::NewTask(Box::new(|| {
        let mut sum = 0;
        for i in 0..30 {
            sum += i;
        }
        println!("Task 3 done: {sum}");
    })));

    event_loop.post(Message::NewTask(Box::new(|| {
        let mut sum = 0;
        for i in 0..40 {
            sum += i;
        }
        println!("Task 4 done: {sum}");
    })));

    event_loop.post_timeout(
        Message::NewTask(Box::new(|| {
            let mut sum = 690;
            for i in (0..69).step_by(3) {
                sum -= i;
            }
            println!("Task 5 done: {sum}");
        })),
        3000,
    );

    worker_threads.stop();
    event_loop.stop();
    println!("Main done");
}

//Sendng terminate-message equal to the amount of threads means that each thread is guranteed to
//finish because we're stacking on new tasks which will wake up waiters. This way we prevent
//threads from waiting indefinitely on a empty task queue, because the thread unfortunately missed
//the cv-signal
enum Message {
    NewTask(Task),
    Terminate,
}
//Declare a type for the task to avoid syntax cluttering
type Task = Box<dyn FnOnce() + Send + 'static>;

/// Arc allows shared variables (mutex. task queue, condvar) to be shared - drops with zero
/// references.
///
/// Declare a task type to avoid needing to specify type each time
///
/// Threads vec is needed to store the threads.
struct Workers {
    tasks: Arc<(Mutex<Vec<Message>>, Condvar)>,
    threads: Vec<Option<thread::JoinHandle<()>>>, //Need to store thread somewhere
}

impl Workers {
    fn new(nr_workers: usize) -> Self {
        let tasks = Arc::new((Mutex::new(Vec::<Message>::new()), Condvar::new()));
        let mut threads = Vec::with_capacity(nr_workers);

        for _ in 0..nr_workers {
            let tasks = Arc::clone(&tasks);

            let thread = thread::spawn(move || {
                loop {
                    let (lock, cvar) = &*tasks;

                    let mut task_queue = lock.lock().unwrap();

                    while task_queue.is_empty() {
                        task_queue = cvar.wait(task_queue).unwrap();
                    }

                    //Assume none-option never occurs
                    let message = task_queue.pop().unwrap();

                    match message {
                        Message::NewTask(task) => {
                            //Releasing lock before running task
                            let _ = drop(lock);
                            task();
                        }

                        Message::Terminate => break,
                    }
                }
            });
            threads.push(Some(thread));
        }

        Workers { tasks, threads }
    }

    fn post(&self, task: Message) {
        let (lock, cvar) = &*self.tasks;

        let mut queue = lock.lock().unwrap();
        queue.push(task);

        cvar.notify_one();
    }

    fn stop(&mut self) {
        let (lock, cv) = &*self.tasks;

        if lock.lock().unwrap().is_empty() {
            for _ in &mut self.threads {
                let mut queue = lock.lock().unwrap();
                queue.push(Message::Terminate);
                cv.notify_one();
            }
        }
    }

    fn post_timeout(&self, task: Message, delay: u64) {
        thread::sleep(Duration::from_millis(delay));
        Workers::post(self, task);
    }
}

impl Drop for Workers {
    fn drop(&mut self) {
        {
            //Pushing terminate-tasks to task-vec with amount equal to amount of threads, that way we guarantee that all threads break the
            //while loop, because they won't be waiting indefinitely on the condvar signal.
            //Notify to wake up already waiting threads
            let (lock, cv) = &*self.tasks;
            for _ in &mut self.threads {
                let mut queue = lock.lock().unwrap();
                queue.push(Message::Terminate);
                cv.notify_one();
            }
        }
        for thread in &mut self.threads {
            if let Some(thread) = thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
