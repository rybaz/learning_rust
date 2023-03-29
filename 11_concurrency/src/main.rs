use std::thread;
use std::sync::mpsc;

fn main() {
    let (s, r) = mpsc::channel();
    let _ = thread::spawn(move ||
    {
        let _ = s.send("Hello from thread");
    });

    match r.recv() {
        Ok(m) => println!("{}", m),
        Err(_) => {}
    }
}
//use std::thread;
//
//fn main() {
//    let value = 20;
//    let handle = thread::spawn(move || -> i32 {
//        value * 2
//    });
//    let result = handle.join();
//    match result {
//        Ok(r) => println!("Result: {}", r),
//        Err(_) => println!("Thread did notfinish successfully")
//    }
//}
