fn foo() {
    let x = Box::new(75);
    println!("`x` points to {}", *x);
}

fn bar() {
    let x = Box::new(75);
    let y = Box::new(42);
    // x = y;         // Not allowed, x is immutable.
    // *x = 43;       // Not allowed, *x is immutable.
    let mut x = Box::new(75);
    x = y;            // OK, x is mutable.
    *x = 43;          // OK, *x is mutable.
}

fn baz() -> Box<i32> {
    let x = Box::new(75);
    x
}

fn main() {
    println!("Hello, world!");
    foo();
    let y = baz();
    println!("baz {}",y);
}
