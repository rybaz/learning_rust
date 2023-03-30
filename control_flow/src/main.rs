fn main() {
    // conditionals
    let condition1 : bool = true;
    let condition2 : bool = false;
    let condition3 : bool = false;

    if condition1 || condition2 && condition3 {
        println!("Good to go!");
    }
    if (condition1 || condition2) && condition3 {
        println!("Good to go!");
    }

    // match
    let animal = "Cat";

    if animal == "Dog" {
        println!("Woof");
    } else if animal == "Cat" {
        println!("Meow");
    } else {
        println!("Unknown");
    }
    
    // ORRRR
    match animal {
        "Dog" => println!("Woof"),
        "Cat" => println!("meow"),
        _ => println!("Unknown") // catch-all if none of the cases match
    }

    // enums
    enum Status {
        Dead,
        Alive
    }

    let (first_name, last_name, status) = ("Charles", "Dickens", Status::Dead);
    match status {
        Status::Alive => println!("{} {} is alive.", first_name, last_name),
        Status::Dead => println!("{} {} is dead.", first_name, last_name),
    }

    // loops
    for i in 0..=10 {
        println!("{}", i)
        
    }

    let array = [1,2,3,4,5];
    for index in 0..array.len() {
        println!("{}", array[index]);
    }

    for data in array.iter() {
        println!("{}", data);
    }

    let mut counter = 0;
    loop {
        println!("{}", counter);
        if counter >= 10 {
            break;
            
        }
        counter += 1;
    }

    fn add_integers(i1 : u8, i2 : u8) -> u8 {
        i1 + i2

        
    }
}
