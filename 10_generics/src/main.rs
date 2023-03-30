use std::{ops::Add, process::Output};

fn main() {
    let result = add_integers(1, 2);
    println!("{}", result);

}

//fn add_integers<T: Add<Output = T>>(i1: T, i2: T) -> T {
//    i1 + i2
//}
// OORRRR
fn add_integers<T>(i1: T, i2: T) -> T
where T: Add<Output = T> {
    i1 + i2
}
