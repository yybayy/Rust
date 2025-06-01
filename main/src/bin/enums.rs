enum colors{
    red,
    black,
    green,
    purple,
    cyan,
    white
}

fn paint_bucket(color: colors){
    match color {
        colors::red => println!("you have a paint bucket filled with the colour red"),
        colors::black => println!("you have a paint bucket filled with the colour black"),
        colors::green => println!("you have a paint bucket filled with the colour green"),
        colors::purple => println!("you have a paint bucket filled with the colour purple"),
        colors::cyan => println!("you have a paint bucket filled with the colour cyan"),
        colors::white => println!("you have a paint bucket filled with the colour white"),
        _ => println!("unfortunately you dropped your bucket"),
    }
    
}

fn main() {
    paint_bucket(colors::red);
}