use std::{
    ops::Range,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use rand::Rng;

fn main() {
    let a: Arc<Mutex<i32>> = Arc::new(Mutex::new(10));
    let mut handles: Vec<JoinHandle<_>> = vec![];

    for i in 0..=100 {
        let clona = a.clone();
        let handle = thread::spawn(move || {
            work(&clona, format!("{}", i).as_str(), 2..5);
        });
        handles.push(handle);
    }

    for i in handles {
        i.join().unwrap()
    }

    println!("Result : {:?}", a.clone().lock().unwrap())
}

fn work(val: &Arc<Mutex<i32>>, prompt: &str, time: Range<u64>) {
    let mut random_rn = rand::thread_rng();
    println!("{} Working .. ", prompt);
    thread::sleep(Duration::from_secs(random_rn.gen_range(time)));
    let mut locked = val.lock().unwrap();
    *locked += random_rn.gen_range(0..10);
    println!("{} Work Result : {}", prompt, *locked);
}
