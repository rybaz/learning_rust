use std::fs;

fn main() {
    let file_handle = fs::File::open("/home/ryan/rust.txt");
    match file_handle {
        Ok(_) => println!("File opened"),
        Err(e) => { dbg!(e); }
    }
}
//use std::fs::File;
//use std::io::{Error, Read};
//    let result = read_file("main.rs");
//    match result {
//        Ok(_) => println!("File opened"),
//        Err(e) => println!("Error opening file: {}", e)
//    }
//}
//
//fn read_file(path: &str) -> Result<String, Error> {
//    let mut file = open_file(path)?;
//    let mut buf = String::new(); 
//    let _ = file.read_to_string(&mut buf);
//
//    Ok(buf)
//}
//
//fn open_file(path: &str) -> Result<File, Error> {
//    File::open(path)
//}


    //let array = [0; 5];
    //println!("{}", array[5]);

    //let vector = vec![1,2,3,4,5];
    //for i in 0..=5 {
    //    let option = vector.get(i);
    //    match option {
    //        None => println!("No data found at index {}.", i),
    //        Some(d) => println!("the data at {} is {}.", i, d)
    //    }
    //}
