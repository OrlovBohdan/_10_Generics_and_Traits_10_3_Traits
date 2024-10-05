#[test]

/*

use std::ops;

// Implement fn multiply to make the code work.
// As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait.
// So, what about `*`?  You can find the answer here: https://doc.rust-lang.org/core/ops/
fn multipl

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");
}
*/

// Implement fn multiply to make the code work.
// As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait.
// So, what about `*`?  You can find the answer here: https://doc.rust-lang.org/core/ops/

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");
}
use std::ops::Mul;

fn multiply<T: Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}


/*
Мы ограничиваем тип T трейтом Mul, который позволяет использовать операцию умножения (*) для типа T.
Output = T означает, что результат умножения будет того же типа, что и сами операнды.
*/

