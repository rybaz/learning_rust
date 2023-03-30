fn main() {
    // ownership blocks message1's value from being used in message2
    // let mut message1 = String::from("Hello, world!");
    // println!("{}", message1);

    // let message2 = message1;
    // println!("{}", message2);

    // borrowing
    let message1 = String::from("Hello, world!");
    println!("{}", message1);

    // assign the heap data to message2
    let message2 = &message1;
    println!("message1 lives at {:p}.", message2);
    println!("Data in message1 is {}", message1);

    // deref message2 to get the actual data
    let message2 = &mut message1;
    *message2 = String::from("Hello");
    println!("{}", message2);


}
