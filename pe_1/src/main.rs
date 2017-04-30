fn main() {
    println!("Project euler problem 1");
    println!("Multiples of 3 and 5");
    println!("If we list all the natural numbers below 10 that are multiples of 3 or 5, \
    we get 3, 5, 6 and 9. The sum of these multiples is 23.  \
    Find the sum of all the multiples of 3 or 5 below 1000.");

    let max_range = 1000;

    let mut sum = 0;
    let mut multiple = 1;
    let mut productof3;
    let mut productof5;

    loop {
        productof3 = 3 * multiple;
        
        if productof3 < max_range
        {
            sum += productof3;
        }
        else {
            break;
        }

        productof5 = 5 * multiple;

        if productof5 < max_range && productof5 % 3 != 0
        {
            sum += productof5;
        }

        multiple = multiple + 1;
    }

    println!("Sum is : {}", sum);
}