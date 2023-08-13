use std::thread;
use std::time::Duration;
use crossbeam::channel::{unbounded, Sender, Receiver, RecvTimeoutError};

fn main() {
    println!("Hello, world!");

    let (sender, receiver) = unbounded();
    
    let sender_thread = thread::spawn(move || {
        sender_fn(sender);
    });

    let receiver_thread = thread::spawn(move || {
        receiver_fn(receiver);
    });

    sender_thread.join().expect("can't join sender thread");
    receiver_thread.join().expect("can't join receiver thread");
}

fn sender_fn(sender: Sender<String>) {
    println!("in sender thread");
    for i in 1..4 {
        let s = format!("Message #{}", i);
        println!("Sender sending message {}", s);
        sender.send(s).unwrap();
        thread::sleep(Duration::from_millis(1200));
    }
    sender.send("EXIT".to_string()).unwrap();
}

fn receiver_fn(receiver: Receiver<String>) {
    println!("in receiver thread");
    loop {
        match receiver.recv_timeout(Duration::from_millis(1000)) {
            Ok(message) => {
                println!("Receiver: received message {}", message);
                if message == "EXIT" {
                    break;
                }
            }
            Err(RecvTimeoutError::Timeout) => {
                println!("Timeout: continuing");
            }
            Err(_) => {
                println!("Channel closed; exiting");
                break;
            }
        }
    }
    println!("Receiver exiting");
}
