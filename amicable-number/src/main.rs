use std::io;

fn main() {
    println!("Enter first & second numbers: ");
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).expect("Failed to read line");
    io::stdin().read_line(&mut b).expect("Failed to read line");

    let a : u32 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a number");
            return;
        }
    };

    let b : u32 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a number");
            return;
        }
    };

    let amicable = are_amicable(a,b);

    if amicable {
        println!("Given number are amicable");
    } else {
        println!("Given number are not amicable");
    }
}

fn are_amicable(a: u32, b: u32) -> bool {
    let mut sum_a = 0;
    let mut sum_b = 0;
    for i in 1..(a/2 + 1) {
        if a % i == 0 {
            sum_a += i;
        }
    }

    for i in 1..(b/2 + 1) {
        if b % i == 0 {
            sum_b += i;
        }
    }

    sum_a == b && sum_b == a
}