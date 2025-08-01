use std::io;

fn main() {
    let respond = "the letter e";
    let mut count = 0;
    loop {
        let mut input = String::new();
        println!(
            "I am the beginning of the end, and the end of time and space. Iam essential to creation, and I sorround every place. What am I?"
        );
        io::stdin().read_line(&mut input).expect("faild to read !");
        count += 1;
        if input.trim() == respond {
            println!("Number of trials: {}", count);
            break;
        }
    }
}
