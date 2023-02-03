fn factorial(num: i32) -> i32 {
    if num == 0 || num == 1 { return 1; }
    return num * factorial(num - 1);
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() > 2 {
        eprintln!("factorial <number>");
        std::process::exit(1);
    }

    if args[0].trim().to_lowercase() != "factorial" {
        eprintln!("factorial <number>");
        std::process::exit(1);
    }

    let number = args[1].trim().parse::<i32>().expect(&format!("{} is not a number", args[1]));

    println!("The factorial of {number} is {}", factorial(number));
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
