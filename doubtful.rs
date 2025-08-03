pub fn doubtful(s: &mut String) {
    s.push('?');
}

fn main() {
    let mut s = "Hello".to_owned();

    println!("Before changing the string: {}", s);

    doubtful(&mut s);

    println!("After changing the string: {}", s);
}
