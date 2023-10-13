// Implement broadcast operation in rust using Message Passing Paradigm
//
// Proposed Solution :
//
// Use a MultiQueue which is a queue (has channel) and has a list of subscribers
// RustRoutines are the abstration equivalent to : Thread + Mailbox (something like Elixir processes)
// Currently RustRoutine stores the main queue tx internally (TODO: extend this in a better way such taht any RustRoutine can send and listen to any Queue)
// Process:
// 1. Create a MultiQueue and start broadcast operation (thread keeps listening to its mailbox and broadcasts to all subscribers except the sender)
// 2. Create RustRoutines
// 3. Start Gossip() and Listening() for RustRoutines
// 4. Gossip() currently waits for rand time and sends msgs to the MultiQueue
// 5. Other RustRoutines listen and print the message

use std::{
    fmt::Display,
    sync::mpsc,
    thread::{self, JoinHandle},
};

#[derive(Clone)]
struct Message<T> {
    sender: String,
    content: T,
}

struct Subscriber<T> {
    name: String,
    tx: mpsc::Sender<Message<T>>,
}

// Queue which enables MPMC
struct MultiQueue<T> {
    tx: mpsc::Sender<Message<T>>,
    rx: mpsc::Receiver<Message<T>>,
    subscriber_mutex: std::sync::Mutex<Vec<Subscriber<T>>>,
}

impl<T> MultiQueue<T>
where
    T: Clone + Send + 'static,
{
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            tx,
            rx,
            subscriber_mutex: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn subscribe(&mut self, name: String, tx_thread: mpsc::Sender<Message<T>>) -> () {
        let mut subscribers = self.subscriber_mutex.lock().unwrap();

        subscribers.push(Subscriber {
            name: name,
            tx: tx_thread,
        });
    }

    pub fn start_broadcast(self) -> JoinHandle<T> {
        let rx = self.rx;
        let subsciber_mutex = self.subscriber_mutex;

        let broadcast_thread = thread::spawn(move || loop {
            let msg = rx.recv().unwrap();

            let subscribers = subsciber_mutex.lock().unwrap();

            for subscriber in subscribers.iter() {
                if msg.sender != subscriber.name {
                    subscriber.tx.send(msg.clone()).unwrap();
                }
            }
        });

        broadcast_thread
    }
}

struct RustRoutine<T> {
    name: String,
    tx: mpsc::Sender<Message<T>>,
    mailbox: mpsc::Receiver<Message<T>>,
    broadcast_queue_mailbox: mpsc::Sender<Message<T>>,
    msg: T,
}

impl<T> RustRoutine<T>
where
    T: Clone + Send + Display + 'static,
{
    pub fn new(name: String, queue: &MultiQueue<T>, msg: T) -> Self {
        let (tx, mailbox) = mpsc::channel();

        Self {
            name,
            tx,
            mailbox,
            broadcast_queue_mailbox: queue.tx.clone(),
            msg,
        }
    }

    pub fn start_listening(self) -> JoinHandle<()> {
        let mailbox = self.mailbox;

        let handle = thread::spawn(move || {
            for msg in mailbox {
                println!("{}: {} from {}", self.name.clone(), msg.content, msg.sender);
            }
        });

        handle
    }

    // Sleep for random interval, wakeup and send some random text to the mailbox in a loop
    pub fn start_gossiping(&self) -> JoinHandle<()> {
        let msg = self.msg.clone();
        let name = self.name.clone();
        let broadcast_queue_mailbox = self.broadcast_queue_mailbox.clone();

        let handle = thread::spawn(move || loop {
            let time_slept = rand::random::<u64>() % 10000;

            thread::sleep(std::time::Duration::from_millis(time_slept));
            broadcast_queue_mailbox
                .send(Message {
                    sender: name.clone(),
                    content: msg.clone(),
                })
                .unwrap();
        });

        handle
    }
}

fn main() {
    // Create MultiQueue - Enables MPMC
    let mut queue = MultiQueue::<String>::new();

    let mut routines = vec![];
    let mut handles = vec![];

    // Create RustRoutines and subs
    for i in 0..5 {
        let name = format!("rust_routine_{}", i);
        let msg = format!("message from {}", i);

        let rust_routine = RustRoutine::new(name.clone(), &queue, msg.to_owned());

        let tx = rust_routine.tx.clone();

        queue.subscribe(name, tx);

        routines.push(rust_routine);
    }

    // Enable Queue broadcast
    let broadcast_thread = queue.start_broadcast();

    // Start RustRoutines to start gossiping and listen to other messages
    for routine in routines {
        let handle = routine.start_gossiping();
        handles.push(handle);

        let handle = routine.start_listening();
        handles.push(handle);
    }

    // Handle Joins
    // Right now everything keeps working indefinitely until the program in terminated manually
    // TODO: How to implement a better way to cleanup ?
    for handle in handles {
        handle.join().unwrap();
    }
    broadcast_thread.join().unwrap();
}
