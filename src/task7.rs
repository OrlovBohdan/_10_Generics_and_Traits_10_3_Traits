#[test]

/*
fn main() {
    assert_eq!(sum(1, 2), 3);
}

// Implement `fn sum` with trait bound in two ways.
fn sum<T>(x: T, y: T) -> T {
    x + y
}
*/


fn main() {
    assert_eq!(sum(1, 2), 3);
}
use std::ops::Add;

// Реализация функции sum с использованием impl Trait
fn sum<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

/*use std::ops::Add;

// Реализация функции sum с использованием where
fn sum<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}

fn main() {
    assert_eq!(sum(1, 2), 3);
}*/


