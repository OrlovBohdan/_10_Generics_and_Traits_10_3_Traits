#[test]

/*

// Fill in the blanks to make it work
fn example1() {
    // `T: Trait` is the commonly used way.
    // `T: Fn(u32) -> u32` specifies that we can only pass a closure to `T`.
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(10), __);
    assert_eq!(cacher.value(15), __);
}


fn example2() {
    // We can also use `where` to construct `T`
    struct Cacher<T>
        where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(20), __);
    assert_eq!(cacher.value(25), __);
}



fn main() {
    example1();
    example2();

    println!("Success!");
}
*/


fn main() {
    example1();
    example2();

    println!("Success!");
}

fn example1() {
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(10), 11);  // Здесь 11
    assert_eq!(cacher.value(15), 11);  // Здесь тоже 11, потому что кэш сохраняет значение
}

fn example2() {
    struct Cacher<T>
    where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(20), 21);  // Здесь 21
    assert_eq!(cacher.value(25), 21);  // Здесь тоже 21, потому что кэш сохраняет значение
}

/*
Чтобы программа работала, необходимо заполнить пропуски ожидаемыми значениями,
которые функция возвращает при вызовах методов value.
Поскольку замыкание, передаваемое в Cacher, просто увеличивает переданное значение на 1, результат будет следующим:

В первом примере для cacher.value(10) результат будет 11, а для cacher.value(15) — также 11, потому что кэшер сохраняет значение первого вызова.
Во втором примере для cacher.value(20) результат будет 21, а для cacher.value(25) — 21 по той же причине.
*/
