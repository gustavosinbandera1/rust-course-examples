
// c++ equivalent 
fn foo(x: i32) -> &'static str {
    let mut result: &'static str;
    if x < 10 {
        result = "less than 10 ";
    } else {
        result = "10 or more "
    }
    return result;
}

fn bar(x: i32) -> &'static str {
    let mut result: &'static str;
    if x < 10 {
        "less than 10 "
    } else {
        "10 or more "
    }
}
 
fn print_all(all: Vec<i32>) {
    for a in all.iter() {
        println!("{}", a);
    }
}

fn print_all2(all: Vec<i32>) {
    for (i, a) in all.iter().enumerate() {
        println!("{}: {}", i, a);
    }
}

fn print_some(x: i32) {
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        y => println!("x is something else {}", y),
    }
}

 fn print_some2(x: i32) {
    match x {
        x => println!("x is something else {}", x)
    }
}
 
fn main() {
    let mut x = 10;
    while x > 0 {
        println!("Current Value: {}", x);
        x -=1;
    }
    print_some(10);
   print_some2(25);
   print_some(14);

    /* loop {
        println!("Just looping");
    } */
println!("---------------------------------------");
let x = 34usize as isize;   // cast usize to isize
    let x = 10 as f32;      // isize to float
    let x = 10.45f64 as i8; // float to i8 (loses precision)
    let x = 4u8 as u64;     // gains precision
    let x = 400u16 as u8;   // 144, loses precision (and thus changes the value)
    println!("`400u16 as u8` gives {}", x);
    let x = -3i8 as u8;     // 253, signed to unsigned (changes sign)
    println!("`-3i8 as u8` gives {}", x);
    //let x = 45 as bool;  // FAILS! (use 45 != 0 instead)
    let x = true as usize;  // cast bool to usize (gives a 1)
   
    println!("Hello, world!");
}
