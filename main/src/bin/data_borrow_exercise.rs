struct Grocery{
    quantity: i32,
    id: i32,
}

fn print_quantity(grocery: &Grocery){
    println!("quantity: {}", grocery.quantity);
}

fn print_id(grocery: Grocery){
    println!("id: {}", grocery.id);
}

fn main() {
    let item = Grocery{
        quantity: 5,
        id: 1,
    };
    
    print_quantity(&item);
    print_id(item);
}