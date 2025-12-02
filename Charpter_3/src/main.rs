mod function;

fn main() {
    // let sum = my_function(5,10);
    // println!("The sum is: {}", sum);
    // let x = 5;
    // println!("The value of x is: {}", x);
    // let x = "six";
    // println!("The value of x is: {}", x);

    // const SUBSCRIBER_COUNT: u32 = 100_000;

// fn my_function(x: i32, y: i32) -> i32 {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
//     x + y

// }

/*
Control Flow


    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
*/

/* 
    Loop 

    let mut counter = 0;
    let result = loop {
        counter += 1; // Increment the counter

        if counter == 10 {
            break counter;
        
        }
    };
    println!("The result is {}", result);
*/


   /*  Classic While Loop

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1; // Decrement the number

        println!("LIFTOFF!!!");
    }
    */

    // For Loop

    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }
} 