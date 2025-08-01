mod scalar;

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    scalar::quo(scalar::diff(f as i16, 32) as f32, 1.8) as f64
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    scalar::pro(scalar::sum(c as u8, 32) as i8, 1.8 as i8) as f64
}

fn main() {
    println!("{} F = {} C", -459.67, fahrenheit_to_celsius(-459.67));
    println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0));
}
