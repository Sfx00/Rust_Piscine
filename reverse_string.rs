pub fn rev_str(input: &str) -> String {
    let reverse = String::from(input);
    reverse.chars().rev().collect()
}

fn main() {
    println!("{}", rev_str("Hello, world!"));
    println!("{}", rev_str("Hello, my name is Roman"));
    println!("{}", rev_str("I have a nice car!"));
    println!("{}", rev_str("How old are You"));
    println!("{}", rev_str("ex: this is an example água"));
}
