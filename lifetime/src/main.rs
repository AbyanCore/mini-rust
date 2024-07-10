fn main() {
    let a = "Hexx".to_string();
    let b = "Bre".to_string();

    let result = compare_long(&a, &b);

    println!("{a} > {b} == {result}")
}

fn compare_long<'a>(a: &'a str,b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

