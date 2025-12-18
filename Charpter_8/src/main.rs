use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);


    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}

//use std::vec;

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let mut v: Vec<i32> = Vec::new();
//     v.push(1);
//     v.push(2);
//     v.push(3);

//     {
//         let v2 = vec![1, 2, 3];
//     }
// }

// Accessing elements in the vector

// fn main() {
//     let mut v = [1, 2, 3, 4, 5];

//     //let third = &v[2]; // Accessing the third element
    
//     for i in &mut v {
//     //println!("{}", i);
//     *i += 50;
//     }

// }


// Strings are stored as a 

    