struct Person {
    name: String,
    age: u8,
}

fn describe(p: &Person) {
    println!("name: {}, age: {}", p.name, p.age);
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 20,
    };
    describe(&peter);

    peter.age = 25;
    describe(&peter);

    let name = String::from("Avery");
    let age = 22;
    let avery = Person { name, age };
    describe(&avery);

    let jackie = Person {
        name: String::from("Jackie"),
        ..avery
    };
    describe(&jackie);
}
