fn main1() {
    let num = 37;

    let mut prime = true;

    if num < 2 {
        prime = false;
    } else {
        for i in 2..num {
            if num % i == 0 {
                prime = false;
                break;
            }
        }
    }

    if prime {
         println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);

    }
}

fn main() {
    // Read a number from the user
    println!("Enter a number:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Convert the input string to a number
    let num: u64 = input.trim().parse().expect("Please enter a valid number");

    if is_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);

    }
    fn is_prime(num: u64) -> bool {
        // Check for 0 and 1
        if num == 0 || num == 1 {
            return  false;
        } 

        // Check for 2 and 3
        if num == 2 || num == 3 {
            return true;
        }

        // Check for numbers greater than 3
        for i in 2..(num / 2 + 1) {
            if num % i == 0 {
                return false;
            }
        }

        return  true;
    }

}
