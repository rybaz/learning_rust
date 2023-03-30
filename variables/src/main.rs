fn main() {
    let my_int : u32 = 0;
    let _my_int : u32 = 0; // will be allocated even though it's not used
    
    //let success : bool = some_function();
    // or
    //let _ = some_function();
    //if success {
        // do something
    //}

    // compile time constant that can only be read
    const MY_CONSTANT : i32 = 99;

    // casting
    let int16 : u16 = 65535;
    print!("{}", int16);

    let int8 : int16 as u8;
    print!("{}", int8);

    // make a vasriable mutable
    let mut int : i32 = 1234;
    int = 4567

}
