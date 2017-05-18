fn main() {
    println!("Sum square difference");
    println!("The sum of the squares of the first ten natural numbers is,
    12 + 22 + ... + 102 = 385
    The square of the sum of the first ten natural numbers is,
    (1 + 2 + ... + 10)2 = 552 = 3025
    Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
    Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.");

    let mut sum_of_squares: u64 = 0;
    let mut square_of_sums: u64 = 0;
    let mut natural_number: u64 = 1;
    
    loop {
        if natural_number > 100
        {
            break;
        }

        sum_of_squares = sum_of_squares + (natural_number * natural_number);
        square_of_sums = square_of_sums + natural_number;

        natural_number = natural_number + 1;
    }

    square_of_sums = square_of_sums * square_of_sums;

    let difference: u64 = square_of_sums - sum_of_squares;
    println!("Difference is {}", difference);
}
