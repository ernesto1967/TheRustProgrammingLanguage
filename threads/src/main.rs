use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    let collection = vec![1, 2, 3, 5, 7, 11, 13, 17, 23];
    let (tx, rx) = mpsc::channel();
    let tx_main = mpsc::Sender::clone(&tx);

    let name = String::from("printer_thread");
    let printer_thread = thread::spawn(move || {
        let my_name = name;
        println!("Spawned thread \"{}\" will print messages", my_name);
        loop {
            let msg = rx.recv();
            match msg {
                Ok(msg) => println!("Message \"{}\" in {}", msg, my_name),
                Err(_) => {
                    println!("Channel closed in {}", my_name);
                    break;
                }
            }
        }
        println!("{} ends now", my_name);
    });
    
    let name = String::from("counter_thread");
    let counter_thread = thread::spawn(move || {
        let my_name = name;
        println!("Spawned thread \"{}\" will iterate over collection {:?}", my_name, collection);
        if true {
            for i in collection {
//                prinboln!("hi number {} from the \"{}\"", i, my_name);
                tx.send(i).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        } else {
            let val = 123;
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("{} ends now", my_name);
    });

    for i in 101..112 {
//        println!("he number {} from main thread", i);
        tx_main.send(i).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    println!("Dropping tx_main");
    drop(tx_main);

    counter_thread.join().unwrap();
    printer_thread.join().unwrap();
}
