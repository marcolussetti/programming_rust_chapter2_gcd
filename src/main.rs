fn main() {
    // println!("Hello, world!");
    println!("{}", gcd(20, 30));
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