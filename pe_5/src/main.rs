fn main() {
    println!("Smallest multiple");
    println!("2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
    What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?");

    //Constraints
    //1. Needs to be a even only, because it's multiple can be both even/odd
    //2. Multiples of max range, this will allow to move larger blocks in the range
    let max_range: u32 = 20;
    let mut index: u32 = 1;

    loop {
        //For every multiple of divider check
        let val: u32 = index * max_range;
        if check_all_multiples(val)
        {
            println!("The lowest multiple between 1:20 is {}", val);
            break;
        }

        index = index + 1;
    }
}

fn check_all_multiples(number: u32) -> bool
{
    let mut index: u32 = 1;
    let max_range: u32 = 20;

    loop
    {
        if index < max_range
        {
            if number % index != 0
            {
                return false;
            }
        }
        else 
        {
            break;
        }

        index = index + 1;
    }

    return true;
}
