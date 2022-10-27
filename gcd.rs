



use std::env;

fn convert(x: String) -> i32 {
    x.parse::<i32>().unwrap()
}

fn gcd(a: i32, b: i32) -> i32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn main() {
    let args: Vec<i32> = env::args().skip(1).map(convert).collect();
    let a = args[0];
    let b = args[1];

    println!("{}", gcd(a, b))
}
