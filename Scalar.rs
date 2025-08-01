pub fn sum(x: u8, y: u8) -> u8 {
    let result = x.checked_add(y);
    match result {
        Some(sum) => sum,
        None => panic!("Error: attempt to add with overflow"),
    }
}
pub fn diff(x: i16, y: i16) -> i16 {
    let result = x.checked_sub(y);
    match result {
        Some(diff) => diff,
        None => panic!("Error: attempt to subtract with overflow"),
    }
}
pub fn pro(x: i8, y: i8) -> i8 {
    let result = x.checked_mul(y);
    match result {
        Some(pro) => pro,pub fn sum(x: u8, y: u8) -> u8 {
    x + y
}
pub fn diff(x: i16, y: i16) -> i16 {
    x - y
}
pub fn pro(x: i8, y: i8) -> i8 {
    x * y
}
pub fn quo(x: f32, y: f32) -> f32 {
    x / y
}
pub fn rem(x: f32, y: f32) -> f32 {
    x % y
}

fn main() {
    // sum
    println!("sum: {}", sum(234, 2)); // 'sum: 236'

    // diff
    println!("diff: {}", diff(234, 2)); // 'diff: 232'

    // product
    println!("pro: {}", pro(23, 2)); // 'pro: 46'

    // quotient
    println!("quo: {}", quo(22.0, 2.0)); // 'quo: 11'

    // remainder
    println!("rem: {}", rem(22.0, 2.0)); // 'rem: 0'
}

        None => panic!("Error: attempt to multiply with overflow "),
    }
}

pub fn quo(x: f32, y: f32) -> f32 {
    x
}
pub fn rem(x: f32, y: f32) -> f32 {
    x % y
}

fn main() {
    // sum
    println!("sum: {}", sum(234, 2)); // 'sum: 236'
    println!("sum: {}", sum(1, 255)); // 'ERROR: attempt to add with overflow'

    // diff
    println!("diff: {}", diff(234, 2)); // 'diff: 232'
    println!("diff: {}", diff(-32768, 32766)); // 'ERROR: attempt to subtract with overflow'

    // product
    println!("pro: {}", pro(23, 2)); // 'pro: 46'
    println!("pro: {}", pro(-128, 2)); // 'ERROR: attempt to multiply with overflow'

    // quotient
    println!("quo: {}", quo(22.0, 2.0)); // 'quo: 11'
    println!("quo: {}", quo(-128.23, 2.0)); // 'quo: -64.115'

    // remainder
    println!("rem: {}", rem(22.0, 2.0)); // 'rem: 0'
    println!("rem: {}", rem(-128.23, 2.0)); // 'rem: -0.22999573'
}
