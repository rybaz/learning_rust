fn main() {
    // math
    let addition : i32 = 23 + 54;
    println!("{}", addition);
    let subtraction : i32 = 32 -12;
    println!("{}", subtraction);
    let multiplication : i32 = 4 * 5;
    println!("{}", multiplication);
    let division : i32 = 321 / 44;
    println!("{}", division);
    let modulus : i32 = 798 % 8;
    println!("{}", modulus);

    // logic
    let int1 : i32 = 20;
    let int2 : i32 = 15;

    if int1 == int2 { println!("Values are equal"); }
    if int1 != int2 { println!("Values are not equal"); }
    if int1 > int2 { println!("int1 is great than"); }
    if int1 < int2 { println!("int2 is less than"); }
    if int1 >= int2 { println!("int1 is greater or equal"); }
    if int1 <= int2 { println!("int1 is less than or equal"); }

    let bool1 = true;
    let bool2 = false;

    if bool1 && bool2 { println!("Both are true"); }
    if bool1 || bool2 { println!("Only one is true"); }

    // bitwise

    let and = int1 & int2;
    println!("{}", and);

    let or = int1 | int2;
    println!("{}", or);

    let xor = int1 ^ int2;
    println!("{}", xor);

    let ls = int1 << int2;
    println!("{}", ls);

    let rs = int1 >> int2;
    println!("{}", rs);
}
