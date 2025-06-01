fn main() {
    let mut x = 5;
    loop {
        if x == 15 {
            println!("x is finnaly {}", x);
            break;
        }
        x += 1;
        println!("X: {}", x);
    }
}