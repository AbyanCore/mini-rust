use std::{ops::Add, time::SystemTime};

use muser::User;

fn adder(x: i32,y: i32) -> i32 {
    return x + y
}

fn main() {
    let mut dx: i8  = 10;

    dx += 120;

    print!("{dx}");
}

#[test]
fn testmain(){
    let res: i32 = adder(10,20);
    println!("{res}");
}

#[test]
fn test_tuple(){
    let mut tup: (i32,f32,bool) = (10,40.2,false);

    println!("{:?}",tup);

    let (a,b,c) = tup;

    println!("{},{},{}",a,b,c);

    tup.0 = 30;

    println!("{:?}",tup);
}

#[test]
fn test_vec(){
    let mut arr: Vec<f32> = Vec::new();

    arr.push(10.0);
    arr.push(30.0);
    arr.push(20.0);
    arr.push(50.0);

    arr.iter().map(move |x| (x + 2.0) / 3.0).for_each(move |x| {
        println!("{x}");
    });
}

#[test]
fn loop_label() {
    let mut number: i32= 1;
    'outer: loop {
        let mut i: i32 = 1;

        loop {
            if number > 10 {
                break 'outer;
            }


            println!("{} x {} = {}",number,i, (number * i));
            i += 1;

            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn array_iteration() {
    let arr: [&str; 3] = ["a","b","c"];

    for x in arr {
        println!("{x}")
    }
}

#[test]
fn range_test() {
    let x = 0..10;

    let _start = x.start;
    let _end = x.end;

    for i in x {
        println!("{i}")
    }
}

fn add_val(data: &mut [String; 3]){
    data[data.len() - 1] = String::from("Hi");
}

#[test]
fn test_ref() {
    let mut data: [String; 3] = [String::from("Hello WOrld"),String::from("Hello World"),String::new()];

    for ix in data.iter() {
        println!("{ix}");
    }

    add_val(&mut data);

    for ix in data.iter() {
        println!("{ix}");
    }
    
}

struct Car {
    name: String,
    brand: String,
    age: u8
}

#[test]
fn create_car() {
    let car_honda: Car = Car { name: String::from("Rail"), brand: String::from("Honda"), age: 10 };

    println!("Name : {}",car_honda.name);
    println!("Brand : {}",car_honda.brand);
    println!("Age : {}",car_honda.age);
    
}

struct Person {
    full_name: String,
    first_name: String,
    last_name: String,
    age: u8
}

impl Person {
    fn print_all(&self){
        println!("
            Full Name: {}
            First Name: {}
            Last Name: {}
            Age: {}\n
        ",self.full_name,self.first_name,self.last_name,self.age);
    }

    fn changefullname(&mut self, new_fullname: String) {
        self.full_name = new_fullname;
    }
}

#[test]
fn pers_struct_imp() {
    let mut pers: Person = Person {
        full_name : String::from("Udin Batako"),
        first_name : String::from("Udin"),
        last_name: String::from("Batako"),
        age: 24
    };

    pers.print_all();

    pers.changefullname(String::from("Hello world"));

    pers.print_all();
}


enum Level {
    Low,
    Medium,
    High
}

#[test]
fn level_test() {

}

enum Payment {
    CreditCard(String),
    BankTransfer(String,String),
    EWallet(String,String)
}

impl Payment {
    fn pay(&self,amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Pembayaran Melalui CreditCard : {} dengan jumlah : {}",number,amount);
            }
            Payment::BankTransfer(bank,number ) => {
                println!("Pembayaran Melalui Bank : {} dan Number : {} dengan jumlah : {}",bank,number,amount);
            }
            Payment::EWallet(wallet,number ) => {
                println!("Pembayaran Melalui Wallet : {} dan Number : {} dengan jumlah : {}",wallet,number,amount);
            }
        }
    }
}

#[test]
fn payment_test() {
    let _payment1: Payment = Payment::CreditCard(String::from("0029933"));
    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("0087622"));
    let _payment3: Payment = Payment::EWallet(String::from("Oren"), String::from("008762883"));

    _payment1.pay(200);
    _payment2.pay(500);
    _payment3.pay(400);

}

mod muser;

#[test]
fn test_mod() {
    let user1: User::Model = User::Model::new(String::from("value"), String::from("value"), String::from("value"), 10);

    user1.print_all();
}

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


#[test]
fn test_point() {
    let point1: Point = Point { x: 10, y: 20 };
    let point2: Point = Point { x: 3, y: 5 };


    println!("x: {} y: {}",point1.x,point1.y);
    println!("x: {} y: {}",point2.x,point2.y);

    let point3 = point1 + point2;

    println!("x: {} y: {}",point3.x,point3.y);
}

fn double(val: Option<i32>) -> Option<i32> {
    match val {
        Some(i) => Some(i * 2),
        None => None,
    }
}

#[test]
fn test_Double() {
    let dob = double(Some(100));

    if dob == None {
        return;
    } 

    print!("{:?}",dob)
}

#[test]
fn getTime() {
    let now = SystemTime::now();

    print!("{:?}",now)
}

#[derive(Debug)]
struct ItemBuf<K,V> {
    key: K,
    value: V   
}

#[test]
fn testdrive() {
    let item1: ItemBuf<&str, &str> = ItemBuf { key: "My Console", value: "Hello Console" };

    println!("{:?}",item1)
}