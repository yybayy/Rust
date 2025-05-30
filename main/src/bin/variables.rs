fn main() {

    //immutable-> bu değişkenler ilk atamalarının ardından sadece readonly değişkenler
    let hello_world = "hello world"; //&str(string slice) != string
    let string:String = "hello world".to_string(); // string
    let h = 'h'; // char
    let byte:i8 = 1; //byte
    let int = 1; // i32
    let float = 1.1; //f64
    let ptr_float = &float;//&f64
    let ptr_int = &int; //&i32

    //mutable-> mut anahtar kelimesiyle değişkenleri readonly durumundan çıkarıp tekrar değer atayabiliyoruz
    let mut mutable_hello_world = "hello world"; //&str
    let mut mutable_string:String = "hello world".to_string(); // string
    let mut mutable_h = 'h'; // char
    let mut mutable_byte:i8 = 1; //byte
    let mut mutable_int = 1; // i32
    let mut mutable_float = 1.1; //f64
    let mut mutable_ptr_float = &float;//&f64
    let mut mutable_ptr_int = &int; //&i32
}