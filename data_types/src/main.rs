fn main() {
    let _int : i8 = -20;   //signed 
    let _uint : u8 = 20;   //unsigned 
    let f32 : f32 = 8.2;  //32-bit float
    println!("32-bit float == {}", f32);
    let f64 : f64 = 3.14; //64-bit float
    println!("32-bit float == {}", f64);
    let char:char = 'A';
    println!("Character == {}", char);

    let _bool:bool = true;


    // arrays
    let array:[i32; 5] = [1,2,3,4,5];
    println!("Second index of array is {}", array[2]);
    let _array:[i32; 1000] = [0; 1000];

    // tuples
    let tuple:(&str, &str, i32) = ("Charles", "Dickets", 1812);
    let (first_name:&str, last_name:&str, dob:i32) = tuple;
    println!("{} {} was born in {}.", tuple.0, tuple.1, tuple.2);

    // strings
    let slice = "Charles Dickens";

    let str = slice.to_string();
    // or
    let str = "Charles Dickens".to_string();
    // or
    let str = String::from(s:"Charles Dickens");

    let slice : (&str) = &str;
    // or
    let slice = str.as_str();

    let first_name = "Ryan";
    let last_name = "Basden";
    let full_name = format!("{} {}", first_name, last_name);
    println!("{}", full_name);
}
