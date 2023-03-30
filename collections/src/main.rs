use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() {
    // vectors
    let mut vector : Vec<u8> = Vec::new();

    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);

    vector.insert(3, 99);
    vector.remove(3);

    for data in vector.iter() {
        println!("{}", data);
    }

    let i = vector.pop();
    match i {
        Some(d) => println!("{}", d),
        None => println!("Not found")
    }

    let result1 : bool = vector.contains(&3);
    let result2 : bool = vector.contains(&99);

    println!("{}", result1);
    println!("{}", result2);

    // maps
    fn hashmaps() {
        let mut map: HashMap<u8, &str> = HashMap::new();

        map.insert(1, "Number 1");
        map.insert(2, "Number 2");
        map.insert(3, "Number 3");
        map.insert(4, "Number 4");
        map.insert(5, "Number 5");

        for kvp in map.iter() {
            println!("Key: {}, Value: {}", kvp.0, kvp.1);
        }

    }

    fn btreemaps() {
        let mut map: BTreeMap<u8, &str> = BTreeMap::new();

        map.insert(1, "Number 1");
        map.insert(2, "Number 2");
        map.insert(3, "Number 3");
        map.insert(4, "Number 4");
        map.insert(5, "Number 5");

        for kvp in map.iter() {
            println!("Key: {}, Value: {}", kvp.0, kvp.1);
    }
    }


    hashmaps();
    btreemaps();
}
