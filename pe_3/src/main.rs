//Using prime trial method.
fn is_prime(num: u64) -> bool {

    //Take the sqrt of it since, if 27 = 9 * 3, and anything smaller than 5 * 5 = 25 would simply
    //have divided it before which is 3 in this case.
    let mut num_sqrt = num as f64;
    num_sqrt = num_sqrt.sqrt();

    let mut divisor = 1;
    let mut is_prime = true;

    loop {
        divisor = divisor + 1;

        if divisor < num_sqrt as u64 + 1 {
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

fn get_prime_series(range: u64, prime_series_ref: &mut Vec<u64>) {
    let mut num = 1;

    loop {
        num = num + 1;

        if num < range {
            if is_prime(num) {
                prime_series_ref.push(num);
            }
        } else {
            break;
        }
    }
}

fn main() {
    println!("Largest Prime Factor");
    println!("The prime factors of 13195 are 5, 7, 13 and 29.
    What is the largest prime factor of the number 600851475143 ?");

    let value = 600851475143;
    let mut max_prime_range = value as f64;
    max_prime_range = max_prime_range.sqrt();

    let mut prime_series: Vec<u64> = Vec::new();

    let mut largest_prime_factor = 1;

    get_prime_series(max_prime_range as u64, &mut prime_series);

    for prime_num in prime_series {
        if value % prime_num == 0 {
            largest_prime_factor = prime_num;
        }
    }

    println!("Largest factor : {}", largest_prime_factor);
}
