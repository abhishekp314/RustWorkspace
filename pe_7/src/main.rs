fn main() {
    println!("10001st prime");
    println!("By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
    What is the 10 001st prime number?");

    //We start from 3 and increment by 2, so that primes are only tested aganist odd numbers
    let mut count = 1;
    let mut next_number = 3;
    let mut next_prime_number = 3;

    loop {
        if is_prime(next_number) {
            next_prime_number = next_number;
            count = count + 1;
        }

        if count == 10001 {
            println!("The 10001st prime number is {}", next_prime_number);
            break;
        }

        next_number = next_number + 2;
    }
}

//Using prime trial method.
fn is_prime(num: u64) -> bool {

    let mut divisor = 1;
    let mut is_prime = true;

    loop {
        divisor = divisor + 1;

        if divisor < num {
            if num % divisor == 0 || num % 2 == 0 {
                is_prime = false;
                break;
            }
        } else {
            break;
        }
    }

    is_prime
}