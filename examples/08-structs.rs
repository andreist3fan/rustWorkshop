struct Person {
    name: String,
    age: u8,
}
impl Person {
    fn print_age(self) {
        println!("{}", self.age);
    }

    fn birthday(&mut self) {
        self.age += 1;
    }

    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    fn static_method() {
        println!("static method");
    }
}
fn main() {
    let mut person = Person {
        name: String::from("Adam"),
        age: 25,
    };
    println!("{}", person.name);
    //person.print_age();
    person.birthday();
    person.print_age();

    let person = Person::new(String::from("Adam"), 25);
    Person::static_method();
}
