fn main() {
    var_mutability();
    compound_types();
    println!("{}", parameter_inputs(69, 'p'));
    control_flow();
    loops();
    closures();
}

// scalar data types and mutability
fn var_mutability() {
    // variable to be tested on the shadow
    let mut x = 6;
    // consts
    const CAPPED_CONSTS: u8 = 13;
    // shadowing
    {
        x *= 2;
        println!("inner scope {x}")
    }
    println!("variable = {x}, constant variable = {CAPPED_CONSTS}");

    // determine number of characters and vectors
    let y = "hello world".len();
    let z = vec!["hi", "yo", "hey"].len();
    println!("string = {}, vector = {:?}", y, z)
}

// compound data types + collection types TODO: hash maps
fn compound_types() {
    // tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{five_hundred} {six_point_four} {one}");
    // arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{first} {second}");
    // vecs
    // empty vector
    let mut a = Vec::new();
    let b = vec![1, 2, 3];
    a.push(69);
    let b_second = b.get(1).unwrap();
    println!("{:?}, {:?}, {:?}", b, a, b_second);
    // compound types work well with loops
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}

// parameters
fn parameter_inputs(value: i32, unit_label: char) -> &'static str {
    println!("The measurement is: {value}{unit_label}");
    "hello"
}

// conditional statements
fn control_flow() {
    let number = 3;

    // if statements
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // match statments
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }
    let x = 4;
    let y = false;
    match x {
        4..=6 if y => println!("yes"),
        _ => println!("no"),
    }

    // if let statements
    let number = Some(13);
    if let Some(i) = number {
        println!("Matched {i:?}!");
    }
}

// loops
fn loops() {
    // normal loop
    let mut counter = 0;
    println!("forever");
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
        println!("and ever!");
    }
    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    // for loop
    let a = [10, 20, 30, 40, 50];
    // 1..10 - 1-9
    // 1..=10 - 1-10
    // .iter() - reads value
    for element in a {
        println!("the value is: {element}");
    }
}

// closures
fn closures() {
    let one = || 13;
    let double = |a| a * 2;
    println!("{}, {}", one(), double(13));

    // FIXME: trying to make a closure that increments a value when a loop continues, it's not working
    // let count = 0;

    // let inc = || {
    //     let mut count += 1
    // };

    // loop {
    //     match inc() {
    //         5 => break,
    //         _ => continue,
    //     }
    // }
}
