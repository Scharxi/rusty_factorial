fn factorial(num: i32) -> i32 {
    if num == 0 || num == 1 { return 1; }
    return num * factorial(num - 1);
}

fn main() {
    println!("{}", factorial(5));
}

#[test]
fn test_factorial() {
    let mut x = 5;
    let mut expected = 120;
    assert_eq!(factorial(x), expected);
    x = 1;
    expected = 1;
    assert_eq!(factorial(x), expected);
}
