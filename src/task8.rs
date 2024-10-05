#[test]

/*

// FIX the errors.
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

struct Unit(i32);

fn main() {
    let pair = Pair{
        x: Unit(1),
        y: Unit(3)
    };

    pair.cmp_display();
}
*/

fn main() {
    // Создаем экземпляр Pair без использования функции new
    let pair = Pair {
        x: Unit(1),
        y: Unit(3),
    };

    // Вызываем метод cmp_display
    pair.cmp_display();
}

use std::fmt;

// Определяем структуру Pair
struct Pair<T> {
    x: T,
    y: T,
}

// Реализация методов для Pair
impl<T> Pair<T> {
    // Удалите или закомментируйте этот метод, если он не нужен
    // fn new(x: T, y: T) -> Self {
    //     Self { x, y }
    // }
}

// Реализация методов для Pair с ограничениями трейтов Debug и PartialOrd
impl<T: fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

// Определяем структуру Unit и реализуем для неё нужные трейты
#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);








/*

*/




