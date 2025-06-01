struct Human {
    name: String,
    age: i32,
    height: u8,
    
}

fn main() {
    let person1 = Human{
        name: "Billy".to_string(),
        age: 14,
        height: 170,
    };
    
    println!("Hello, my name is {}, im {} centimeters tall and\
      im {} years old!", person1.name,  person1.height, person1.age);
}