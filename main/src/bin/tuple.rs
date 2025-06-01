fn get_coordinates() -> (i32, i32){
    (4,7)
}




fn main() {
let (x,y) = get_coordinates();
    if y > 5 { 
        println!("y axis is greater than 5");
    }
    else if y < 5 { 
        println!("y axis is less than 5");
    }
    else { 
        println!("y axis is 5");
    }
}