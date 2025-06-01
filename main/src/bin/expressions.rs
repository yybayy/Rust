use std::io::{stdin, stdout, Write};

fn size_finder (is_big:bool){
    if is_big{
       println!("oi das veriy beaigh");
    }
    else {
        println!("ur numba is smol");
    }
}


fn main() {
    let mut input_number = String::new();
    
    print!("Enter a number: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut input_number)
        .expect("failed to read line") ;
    
    let number: i32 = input_number.trim().parse().expect("not a number");


    let is_number_big =
        if number >= 100 {
            true
        }
        else {
            false
        };

    size_finder(is_number_big);




}