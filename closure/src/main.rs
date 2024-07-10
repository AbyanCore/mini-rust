use std::thread;
use std::time::Duration;

fn main() {
    high_calculation_algorithm_v3(10,30);
}

fn high_calculation_algorithm_v2(intensity: u8,value: i32) {
    let calculation = |val: i32| -> i32 {
        println!("calculating...");
        thread::sleep(Duration::new(1,0));
        println!("+{}",val);
        val
    };
    let mut sum: i32 = 0;

    let mut li = 0;
    loop {
        if li >= intensity {break};

        sum += calculation(value);

        li += 1;
    }

    println!("Finish : {}",sum);
}

struct Cacher<T>
where T: Fn(i32) -> i32 {
    calculation: T,
    value: Option<i32>
}

impl<T> Cacher<T>
where T: Fn(i32) -> i32 {
    pub fn new(calculation: T) -> Cacher<T>{
        Cacher { calculation: calculation, value: None }
    }
}

fn high_calculation_algorithm_v3(intensity: u8,value: i32) {
    let calculation = |val: i32| -> i32 {
        println!("calculating...");
        thread::sleep(Duration::new(1,0));
        println!("+{}",val);
        val
    };
    let mut sum: i32 = 0;

    let mut li = 0;
    loop {
        if li >= intensity {break};

        sum += calculation(value);

        li += 1;
    }

    println!("Finish : {}",sum);
}
