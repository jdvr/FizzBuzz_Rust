// 3 -> Fizz
// 5 -> Buzz
// Both  -> FizzBuzz
// _ -> number

enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Other(u32),
}

fn process(number: u32) -> FizzBuzz {
    match number % 3 {
        0 => match number % 5 {
            0 => FizzBuzz::FizzBuzz,
            _ => FizzBuzz::Fizz,
        },
        _ => match number % 5 {
            0 => FizzBuzz::Buzz,
            _ => FizzBuzz::Other(number),
        },
    }
}

fn main() {
    for number in 1..20 {
        match process(number) {
            FizzBuzz::Fizz => println!("Fizz"),
            FizzBuzz::Buzz => println!("Buzz"),
            FizzBuzz::FizzBuzz => println!("FizzBuzz"),
            FizzBuzz::Other(n) => println!("{}", n),
        }

    }
}
