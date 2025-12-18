// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("Added to waitlist");
//         }

//         pub fn seat_at_table() {
//             println!("Seated at table");
//         }
//     }


//     mod serving {
//         fn take_order() {
//             println!("Order taken");
//         }

//         fn serve_order() {
//             println!("Order served");
//         }

//         fn take_payment() {
//             println!("Payment taken");
//         }
//     }
// }

//Example of making a module public and using absolute and relative paths

// mod front_of_house {
//     pub mod hosting {
//     pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();   
// }

