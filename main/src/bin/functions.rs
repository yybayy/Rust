fn main() {
print_name("ahmet".parse().unwrap(), "mehmet".parse().unwrap());
println!("{:?} and {:?}", math_i32(312,124), math_f64(2.7,4.8));
}

fn print_name(name:String, surname:String) {
    println!("Hello, my name is {name} {surname}");
}

fn math_i32(a:i32, b:i32) -> i32 {
    a + b
}

fn math_f64(a:f64, b:f64) -> f64  {
    a + b
}