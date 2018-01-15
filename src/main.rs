use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
        
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ..." ).unwrap();
        std::process::exit(1);
    }s

    let mut running_gcd = numbers[0];
    for number in &numbers[1..] {
        gcd = gcd(gcd, *number);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d)

    // println!("Hello, world!");
    //println!("{}", gcd(20, 30));
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0); // check neither is zero
    while a != 0 {
        if a < b { // swap them
            let temp = a;
            a = b;
            b = temp;
        }
        a = a % b;
    }
    b
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
    3 * 11);
}