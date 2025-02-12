#[derive(Debug)]
// struct MyValues<T, U> {
//     foo : T,
//     bar: U,
// }

struct Person {
    name: String,
    age: u8,
}

fn main() {
    // let first_struct: MyValues<i32> = MyValues { foo: 1 };
    // let second_struct: MyValues<String> = MyValues { foo: String::from("Hello") };

    // println!("{:?}", first_struct);
    // println!("{:?}", second_struct);

    encode();
}

fn test() {
    let mut s1: String = String::from("abc");

    let s2 = s1.clone();

    println!("{} {}", s1, s2);
}

fn encode() {
    let init_person : Person = Person {
        name: "Alice".to_string(),
        age: 20,
    };

    let encoded_data = init_person.try_to_vec().unwrap();

    println!("{:?}", encoded_data);


}

fn decode(encoded_data: Vec<u8>) -> Person {
    let decoded_data = Person::try_from_slice(&encoded_data).unwrap();

    decoded_data
}
