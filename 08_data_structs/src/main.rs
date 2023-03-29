fn main() {
    let sheep = Sheep::new(String::from("Babe"));
    sheep.make_noise();

    impl AnimalTraits for Sheep {
        fn make_noise(&self) {
            println!("baaaa");
        }
    }

    struct Sheep {
        name: String
    }

    trait AnimalTraits {
        fn make_noise(&self);
    }

}



//    // make rectangle a variable with a struct from line 16
//    let rectangle = Rectangle {
//        length: 20,
//        width: 15
//    };
//
//    // call function from line 13 on variable rectangle and store it in "area"
//    let area = rectangle.calculate_area();
//    // print the dimensions and the area
//    println!("Length: {}, Width: {}, Area: {}",
//        rectangle.length, rectangle.width, area);
//}
//
////create the Rectangle struct with u16 values
//struct Rectangle {
//    length: u16,
//    width: u16
//}
//
//// attach a function to a Rectangle struct that we can use
//impl Rectangle {
//    fn calculate_area(&self) -> u16 {
//        self.length * self.width
//    } 
//}


//    enum Status {
//        ALIVE,
//        DEAD
//    } 
//
//    struct Person {
//        first_name: String,
//        last_name: String,
//        date_of_birth: u16,
//        status: Status
//    }
//
//    let mut person = Person {
//        first_name: String::from("Charles"),
//        last_name: String::from("Dickens"),
//        date_of_birth: 1812,
//        status: Status::ALIVE
//    };
//
//    person.kill();
//
//    match person.status {
//        Status::ALIVE => println!("{} {} is alive", person.first_name, person.last_name),
//        Status::DEAD => println!("{} {} is dead", person.first_name, person.last_name)
//    }
//
//    println!("{} {} was born in {}.", person.first_name, person.last_name, person.date_of_birth); 
//
//    impl Person {
//        fn kill(&mut self) {
//            self.status = Status::DEAD;
//        }
//    }
//}
