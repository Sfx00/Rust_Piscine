pub fn factorial2(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => num * factorial2(num - 1),
    }
}

pub fn factorial(num: u64) -> u64 {
    let mut res: u64 = 1;
    let mut i = 1;
    if num == 1 || num == 0 {
        return res;
    }
    while i != num {
        res *= i + 1;
        i += 1;
    }
    res
}
fn main() {
    println!("The factorial of 0 = {}", factorial2(0));
    println!("The factorial of 1 = {}", factorial(1));
    println!("The factorial of 5 = {}", factorial2(5));
    println!("The factorial of 10 = {}", factorial(10));
    println!("The factorial of 19 = {}", factorial2(19));
}
