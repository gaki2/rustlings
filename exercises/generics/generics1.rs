// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

impl<K: std::fmt::Display> Wrapper<K> {
    pub fn value(value: K) -> () {
        println!("{}", value);
    }
}

fn main() {
    let mut shopping_list: Vec<String> = Vec::new();
    Wrapper::value(42);
    shopping_list.push(String::from("milk"));
}
