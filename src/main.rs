mod user;

#[derive(Debug, PartialEq)]
enum FizzBuzzResult {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonnaci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonnaci(n - 1) + fibonnaci(n - 2)
    }
}

fn fizzbuzz(n: u32) -> FizzBuzzResult {
    if n % 15 == 0 {
        return FizzBuzzResult::FizzBuzz;
    } else if n % 3 == 0 {
        return FizzBuzzResult::Fizz;
    } else if n % 5 == 0 {
        return FizzBuzzResult::Buzz;
    }

    FizzBuzzResult::Number(n)
}

fn fizzbuzz2(n: i32) -> FizzBuzzResult {
    match (n % 3, n % 5) {
        (0, 0) => FizzBuzzResult::FizzBuzz,
        (0, _) => FizzBuzzResult::Fizz,
        (_, 0) => FizzBuzzResult::Buzz,
        _ => FizzBuzzResult::Number(n as u32),
    }
}

fn is_palindrome(s: String) -> bool {
    let t: String = s.chars().rev().collect();
    return s == t;
}

fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn prime_numbers(n: u32) -> Vec<u32> {
    let mut primes = vec![];
    let mut i = 2;

    while primes.len() < n as usize {
        if is_prime(i) {
            primes.push(i);
        }

        i += 1;
    }

    primes
}

fn main() {
    let user = user::User::new(
        1,
        "Alex".to_string(),
        "a@a.com".to_string(),
        "123".to_string(),
    );
    let name = "Alex";
    println!("Hello, {name}!");
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    println!("{}", fibonnaci(3));
    println!("{:?}", fizzbuzz(5));
    println!("{:?}", fizzbuzz2(18));
    println!("{}", factorial(3));
    println!("{:?}", prime_numbers(5));
    println!("{}", is_palindrome("hannah".to_string()));
    println!("{:?}", user);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_fibonnaci() {
        assert_eq!(super::fibonnaci(0), 0);
        assert_eq!(super::fibonnaci(1), 1);
        assert_eq!(super::fibonnaci(2), 1);
        assert_eq!(super::fibonnaci(3), 2);
        assert_eq!(super::fibonnaci(4), 3);
        assert_eq!(super::fibonnaci(5), 5);
        assert_eq!(super::fibonnaci(6), 8);
        assert_eq!(super::fibonnaci(7), 13);
        assert_eq!(super::fibonnaci(8), 21);
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(super::fizzbuzz(3), super::FizzBuzzResult::Fizz);
        assert_eq!(super::fizzbuzz(5), super::FizzBuzzResult::Buzz);
        assert_eq!(super::fizzbuzz(15), super::FizzBuzzResult::FizzBuzz);
        assert_eq!(super::fizzbuzz(17), super::FizzBuzzResult::Number(17));
    }

    #[test]
    fn test_fizzbuzz2() {
        assert_eq!(super::fizzbuzz2(3), super::FizzBuzzResult::Fizz);
        assert_eq!(super::fizzbuzz2(5), super::FizzBuzzResult::Buzz);
        assert_eq!(super::fizzbuzz2(15), super::FizzBuzzResult::FizzBuzz);
        assert_eq!(super::fizzbuzz2(17), super::FizzBuzzResult::Number(17));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(super::factorial(0), 1);
        assert_eq!(super::factorial(1), 1);
        assert_eq!(super::factorial(2), 2);
        assert_eq!(super::factorial(3), 6);
        assert_eq!(super::factorial(4), 24);
        assert_eq!(super::factorial(5), 120);
        assert_eq!(super::factorial(6), 720);
        assert_eq!(super::factorial(7), 5040);
        assert_eq!(super::factorial(8), 40320);
    }

    #[test]
    fn test_prime_numbers() {
        assert_eq!(super::prime_numbers(0), vec![]);
        assert_eq!(super::prime_numbers(5), vec![2, 3, 5, 7, 11]);
        assert_eq!(
            super::prime_numbers(10),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
        assert_eq!(
            super::prime_numbers(15),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
        );
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(super::is_palindrome("hannah".to_string()), true);
        assert_eq!(super::is_palindrome("racecar".to_string()), true);
        assert_eq!(super::is_palindrome("hello".to_string()), false);
        assert_eq!(super::is_palindrome("world".to_string()), false);
        assert_eq!(super::is_palindrome("a".to_string()), true);
        assert_eq!(super::is_palindrome("aa".to_string()), true);
    }
}
