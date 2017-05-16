fn main() {
    println!("Largest palindrome product");
    println!("A palindromic number reads the same both ways. 
    The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
    Find the largest palindrome made from the product of two 3-digit numbers.");

    let mut largest: u32 = 0;

    let mut a: u32 = 999;
    loop {
        let mut b: u32 = 999;

        loop {
            let product: u32 = a * b;
            if product > largest && is_palindrome(product) {
                println!("Large Palindrome product found {} * {} = {}", a, b, product);
                largest = product;
            }

            b = b - 1;

            if b < 100 {
                break;
            }
        }
        a = a - 1;

        if a < 100 {
            break;
        }
    }

    println!("Largest Palindrome product is {}", product);
}

fn is_palindrome(number: u32) -> bool {
    let mut val = number;
    let mut mirrored_number = 0;
    loop {
        if val == 0 {
            break;
        }

        mirrored_number = (mirrored_number * 10) + val % 10;
        val = val / 10;
    }

    if mirrored_number == number {
        return true;
    }

    return false;
}