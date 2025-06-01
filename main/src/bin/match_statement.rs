fn main() {
    let situation = false;
    
    match situation {
        true => println!("Situation is true"),
        false => println!("Situation is false"),
    }
    
    let some_number = 5;
    
    match some_number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("No one"),
    }
}