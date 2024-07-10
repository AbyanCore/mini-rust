use std::{thread, time::Instant};

fn main() {
    let start = Instant::now();
    let result = thread::spawn(|| {
        match reqwest::blocking::get("https://crates.io/crates/cargo-renamepkg") {
            Ok(ix) => ix.text().unwrap(),
            Err(_) => "Error".to_string(),
        }
    })
    .join()
    .unwrap();

    println!("Result: {}", result);
    println!("Time Elapsed: {:?}", start.elapsed());
}
