use std::io;


#[derive(Debug)]
enum Flavor{
    Empty,
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink{
    flavor: Flavor,
    fluid_oz: f32,
}


fn main() {
    let mut flavor: String = String::new();

    println!("Hello kind sir, what would you like to drink?\n\
    (options: Sparkling, Sweet, Fruity");

    io::stdin().read_line(&mut flavor).expect("Failed to read line");

    let drink = match flavor.trim() {
        "Sparkling" => Drink 
        {
            flavor: Flavor::Sparkling,
            fluid_oz: 6.5
        },
        "Sweet" => Drink 
        {
            flavor: Flavor::Sweet,
            fluid_oz: 4.3
        },
        "Fruity" => Drink 
        {
            flavor: Flavor::Fruity,
            fluid_oz: 9.9
        },
        _ => Drink 
        {
            flavor: Flavor::Empty, 
            fluid_oz: 0.0
        },
    };
    
    println!("Here is your {:?} drink sir with {:?} oz amount", drink.flavor, drink.fluid_oz);





}