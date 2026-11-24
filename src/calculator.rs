pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

pub fn calculate(option: i32, a: i32, b: i32) -> i32 {
    match option {
        1 => add(a, b),
        2 => subtract(a, b),
        3 => multiply(a, b),
        4 => divide(a, b),
        _ => panic!("Invalid option"),
    }
}