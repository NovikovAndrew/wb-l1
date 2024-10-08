fn main() {
    let person = &Person{name: String::from("NAME")};
    person.say()
}

trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name)
    }
}
