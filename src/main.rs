// 3 -> Fizz
// 5 -> Buzz
// Both  -> FizzBuzz
// _ -> number

fn main() {
    for number in 1..20 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }
}
