use autorusttest::add_one;
use autorusttest::{PrimaryColor, SecondaryColor, mix};

fn main() {
    println!("Hello, world!");

    let y = add_one(5);
    println!("The result is {}", y);

    let primary_color1 = PrimaryColor::Red;
    let primary_color2 = PrimaryColor::Blue;
    let answer = mix(primary_color1, primary_color2);

    if answer == SecondaryColor::Purple {
        println!("The result is Purple as expected");
    } else {
        println!("The result is not Purple");
    }
}
