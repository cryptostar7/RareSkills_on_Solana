/**
 * Rust Struct
 */

// struct Person {
//     age: u8,
//     name: String,
// }

// impl Person {
//     fn new(name: String, age: u8) -> Self {
//         Person {name, age}
//     }

//     fn can_drink(&self) -> bool {
//         if self.age >= 21 as u8 {
//             return true;
//         }
//         return false;
//     }

//     fn age_in_one_year(&self) -> u8 {
//         return &self.age + 1;
//     }
// }

// fn main() {
//     let person: Person = Person::new(String::from("Alice"), 19);

//     println!("{}", person.can_drink()); // false
//     println!("{:?}", person.age_in_one_year()); // 20
//     println!("{:?}", person.name);

// }

/**
 * Rust Trait
 */

// trait Speed {
//     fn get_speed_kph(&self) -> f64;
// }

// struct Car {
//     speed_mph: f64,
// }

// struct Boat {
//     speed_knots: f64,
// }

// impl Speed for Car {
//     fn get_speed_kph(&self) -> f64 {
//         // Convert miles per hour to kilometers per hour
//         self.speed_mph * 1.60934
//     }
// }


// // We also implement the `Speed` trait for `Boat`
// impl Speed for Boat {
//     fn get_speed_kph(&self) -> f64 {
//         // Convert knots to kilometers per hour
//         self.speed_knots * 1.852
//     }
// }

// fn main() {
//     // Initialize a `Car` and `Boat` type
//     let car = Car { speed_mph: 60.0 };
//     let boat = Boat { speed_knots: 30.0 };

//     // Get and print the speeds in kilometers per hour
//     let car_speed_kph = car.get_speed_kph();
//     let boat_speed_kph = boat.get_speed_kph();

//     println!("Car Speed: {} km/h", car_speed_kph); // 96.5604 km/h
//     println!("Boat Speed: {} km/h", boat_speed_kph); // 55.56 km/h
// }

use macro_demo::*;

#[foo_bar_attribute]
struct MyStruct {
    baz: i32,
}

fn main() {
    let demo = MyStruct::default();

    println!("struct is {:?}", demo);

    let double_foo = demo.double_foo();

    println!("double foo: {}", double_foo);
}