use std::io::stdin;

#[derive(Debug)]
pub enum MATH {
    Plus(f32, f32),
    Sub(f32, f32),
    Divide(f32, f32),
    Multi(f32, f32),
    Empty,
}

fn get_number(promt: &str) -> Result<f32, String> {
    let mut buffer = String::new();
    println!("{} : ", promt);
    match stdin().read_line(&mut buffer) {
        Ok(_) => {
            let num = buffer.trim().parse::<f32>();
            match num {
                Ok(val) => Ok(val),
                Err(_) => Err("Invalid Number".to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn math_engine(val: &mut MATH) -> Result<(), String> {
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(_) => {
            let operator = buffer.trim();
            *val = match operator {
                "+" => MATH::Plus(get_number("Value 1 : ")?, get_number("Value 2 : ")?),
                "-" => MATH::Sub(get_number("Value 1 : ")?, get_number("Value 2 : ")?),
                "/" => MATH::Divide(get_number("Value 1 : ")?, get_number("Value 2 : ")?),
                "*" => MATH::Multi(get_number("Value 1 : ")?, get_number("Value 2 : ")?),
                _ => return Err("Not Found".to_string()),
            };
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    println!("Welcome To My Calculator (Supported *,/,+,-)");
    let mut val: MATH = MATH::Empty;

    match math_engine(&mut val) {
        Ok(_) => match val {
            MATH::Plus(val1, val2) => println!("{} + {} : {}", val1, val2, val1 + val2),
            MATH::Sub(val1, val2) => println!("{} - {} : {}", val1, val2, val1 - val2),
            MATH::Multi(val1, val2) => println!("{} * {} : {}", val1, val2, val1 * val2),
            MATH::Divide(val1, val2) => println!("{} / {} : {}", val1, val2, val1 / val2),
            MATH::Empty => println!("Nothing Can Do"),
        },
        Err(err) => println!("{}", err),
    }
}
