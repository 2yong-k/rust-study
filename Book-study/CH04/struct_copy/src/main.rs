#[derive(Clone)]    // Clone 트레잇 사용
struct Person {
    name: String,
    age: i32,
}
impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self { name: name.to_string(), age }
    }
}

fn main() {
    let alex = Person::new("Alex", 18);

    // ERROR: 소유권 이전으로 불가
    // let mut betty = alex
    // betty.name = String::from("Betty");

    let mut betty = alex.clone();
    betty.name = String::from("Betty");

    // let betty = Person {
    //     name: String::from("Betty"),
    //     ..alex
    // };
    println!("{},{}", alex.name, alex.age);
    println!("{},{}", betty.name, betty.age);
}