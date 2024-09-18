use std::io;

fn main() {
    let mut x = 4; //mut means mutable, so we can change it, could also do let x = 5 instead of making it mutable
    println!("x is: {}", x);

    {  // define a different scope -- called shadowing
        let x = 2; //Different scope, so the var is local to this scope
        println!("x is: {}", x);
    }


    x = x + 1;
    println!("x is: {}", x);

    const SECONDS: u32 = 60; // Constant variable, cannot be changes, requires a data type
    println!("Seconds: {}", SECONDS);

    //can also define like this:
    let data = 50.50f32;
    println!("{}", data);


    let n: i32 = 2; // assign a data type, could use i8, i16, i32 (signed int), u32 (unsigned int), f32, f64(float) etc
    println!("integer: {}", n);

    let floating: f32 = 10.9;
    println!("Float: {}", floating);

    let true_or_false: bool = true; //true or false
    println!("bool: {}", true_or_false);

    let letter: char = 'a';
    println!("char: {}", letter);
    //tuples
    let mut tup: (i32, bool, char) = (1, true, 'a'); // tuple, can access by tup.0, tup.1, tup.2 etc
    let tup2: (i8, bool, char) = (9, false, '2');

    println!("{}", tup.1);
    println!("{}", tup2.0);

    tup = (2, false, 'c');

    println!("{}", tup.1);

    //array
    let mut arr = [1, 2, 3, 4, 5];  //needs to be the same type
    println!("{}", arr[4]);
    arr[4] = 3;
    println!("{}", arr[4]);

    let arr2: [char; 4] = ['a', 'b', 'c', 'd']; // assigning datatype and number of elements to char and 4

    println!("{}", arr2[0]);

    let value1: u8 = 4;
    let value2 = value1;
    println!("{} + {} = {}", value1, value2, value1+value2); //print multiple vars


    //input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line"); //& is a reference, typically just (& input) would be immutable, so we add mut so we can change it
    println!("{}", input);

    //type casting

    let floatx = 12.0f32;
    let floaty = 10.0f64;
    //These cannot be added together, as they are not the same type, they need to be cast to the same type

    let floatz = (floatx as f64) / floaty;
    println!("{}", floatz);
    println!("{}", i32::MAX); //can get highest number for that data type
}
