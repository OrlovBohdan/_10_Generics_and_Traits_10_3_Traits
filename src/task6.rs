#[test]

/*

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
// FIX the errors here, you can make a fake random, or you can use trait object.
fn random_animal(random_number: f64) -> impl Animal {
    if random_number < 0.5 {
        Sheep {}
    } else {
        Cow {}
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
*/
fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

/*
Ваша функция random_animal не может возвращать разные типы (такие как Sheep и Cow) из-за того, что impl
Trait не поддерживает возвращение нескольких типов. Для решения этой проблемы можно использовать трейтовые
объекты (например, Box<dyn Animal>), которые позволяют возвращать значения разных типов, реализующих один и тот же трейт.

Использование Box<dyn Animal>: В функции random_animal возвращаемый тип изменён на Box<dyn Animal>.
Это позволяет возвращать указатель на объект, который может быть либо Sheep, либо Cow, реализующий трейт Animal.
Использование Box::new(): Объекты Sheep и Cow оборачиваются в Box, чтобы создать трейтовый объект.
*/

