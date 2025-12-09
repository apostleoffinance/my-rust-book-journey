#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle{
        width: 10,
        height: 40,
    };

    let rect2 = Rectangle{
        width: 60,
        height: 45,
    };

    let rect3 = Rectangle::square(20);

    println!("rect can hold rect1:{}", rect.can_hold(&rect1));
    println!("rect can hold rect2:{}", rect.can_hold(&rect2));


    
    println!("rect: {:#?}", rect);

    println!("The area of the rectangle is {} square pixels.", rect.area());

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }   
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

   

    // let mut user1 = User {
    //     email: String::from("ola@men.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let name = user1.username;
    // user1.username = String::from("ola123");

    // let user2 = build_user(
    //     String::from("kyle@mail.com"),
    //     String::from("kyle456")
    // );

    // let user3 = User {
    //     email: String::from("jame@mail.com"),
    //     username: String::from("james123"),
    //     ..user2
    // };

}


// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }