use std::thread;

fn main() {
    let x = Worker::new(1, 50);
    let y = Worker::new(2, 50);

    loop {}
}
struct Worker {
    id: u32,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32, number: u32) -> Self {
        Worker { id, thread }
    }
}

mod client {

    fn create_connection() {}
}

mod server {

    fn read_request() {}
}
