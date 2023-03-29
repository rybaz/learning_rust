use rand::random;

fn main() {
    let rand = random::<u32>();
    println!("Your random number is: {}", rand);
    
}

//mod calc;
//
//fn main() {
//    let result1 = calc::add(10, 5);
//    println!("{}", result1);
//    let result2 = calc::subtract(10, 5);
//    println!("{}", result2);
//
//}
