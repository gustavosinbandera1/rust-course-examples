fn foo(_x: &'static str) -> &'static str {
    "world"
}

fn main() {
    //let world = "world gustavo";
    let world: &'static str = "defining the type of variable";
    //let world: &'static str = "world";
    println!("Hello {}!", world);
    println!("Hello 2 {}!", foo("bar"));
    // println!("Hello, world! Open source world ");
}
