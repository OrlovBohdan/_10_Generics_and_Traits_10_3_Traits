#[test]


/*

// Implement `fn summary` to make the code work.
// Fix the errors without removing any code line
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn main() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(post);
    summary(weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}

// Implement `fn summary` below.

*/

fn main() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };

    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post); // передаём ссылку на post
    summary(&weibo); // передаём ссылку на weibo

    println!("{:?}", post);
    println!("{:?}", weibo);
}

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
#[allow(dead_code)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post '{}' is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo: '{}'", self.username, self.content)
    }
}

// Implement `fn summary` below.
fn summary(item: &impl Summary) {
    println!("{}", item.summarize());
}

/*
Ошибка возникает из-за того, что структура Post не реализует трейт Copy, и когда вы передаёте её в функцию summary,
происходит перемещение значения. Это означает, что после вызова summary(post) вы больше не можете использовать переменную post,
потому что она была "перенесена" в функцию.

Чтобы избежать этой проблемы, можно изменить функцию summary, чтобы она принимала ссылку на объект,
а не само значение. Для этого нужно указать, что параметр item — это ссылка на объект, реализующий трейт Summary.
*/




