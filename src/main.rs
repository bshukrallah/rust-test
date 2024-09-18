use std::io::{self, Write};

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

    //*********
    //tuples
    //*********

    let mut tup: (i32, bool, char) = (1, true, 'a'); // tuple, can access by tup.0, tup.1, tup.2 etc
    let tup2: (i8, bool, char) = (9, false, '2');

    println!("{}", tup.1);
    println!("{}", tup2.0);

    tup = (2, false, 'c');

    println!("{}", tup.1);

    //*********
    //array
    //*********

    let mut arr = [1, 2, 3, 4, 5];  //needs to be the same type
    println!("{}", arr[4]);
    arr[4] = 3;
    println!("{}", arr[4]);

    let arr2: [char; 4] = ['a', 'b', 'c', 'd']; // assigning datatype and number of elements to char and 4

    println!("{}", arr2[0]);

    let value1: u8 = 4;
    let value2 = value1;
    println!("{} + {} = {}", value1, value2, value1+value2); //print multiple vars

    //*********
    //input
    //*********

    let mut input = String::new();
    print!("Hi what is your name? "); // print to not create a new line
    io::stdout().flush().unwrap(); //ensure the prompt is displayed on the same line
    io::stdin().read_line(&mut input).expect("failed to read line"); //& is a reference, typically just (& input) would be immutable, so we add mut so we can change it
    println!("Hello {}", input);


    //*********
    //type casting
    //*********

    let floatx = 12.0f32;
    let floaty = 10.0f64;
    //These cannot be added together, as they are not the same type, they need to be cast to the same type

    let floatz = (floatx as f64) / floaty;
    println!("{}", floatz);
    println!("{}", i32::MAX); //can get highest number for that data type

    //*********
    // input conversions
    //*********

    input.clear(); // clear this var since it had the input from above
    print!("Input a number to add to 5: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("expected to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{} + 5 = {}", input.trim(), int_input+5);

    //.trim() remove leading and trailing chars
    //.parse() used to convert a string to another type
    input.clear(); // clear this var since it had the input from above
    print!("Enter a string to check the length of: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("expected to read line");
    
    //let trimmed_input = input.trim();
    let s_len = calculate_length(&input);
    println!("The length of '{}' is {}", input.trim(), s_len);

    //*********
    //conditions
    //*********
    println!("{}",  (2 as f32) >= 2.2); //comparing the float 2, to 2.2. must be of the same type.
    println!("{}", true && 1 < 100); //& = AND, || = OR, ! = NOT
    println!("{}", !(1>100)); // We're checking not here, so NOT(condition), in this case it's true

    let answer = 2;
    if answer == 1 {
        println!("Correct");
    }
    else if answer == 2 {
        println!("2");
        if answer != 3 {
            println!("not 3"); //nested
        }
    } 
    else {
        println!("Wrong");
    }

}


// -> return type, could String or i32, etc, use usize or isize for sizes, memory or indexing
fn calculate_length(s: &str) -> usize {
    s.trim().len()
}