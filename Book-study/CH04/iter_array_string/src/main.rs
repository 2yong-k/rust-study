fn main() {
    let array = [
        "Apple".to_string(), 
        "Banana".to_string(), 
        "Mango".to_string(), 
        "Tomato".to_string()
    ];

    // for a in array { // 여기서 소유권이 이동한다
    //     println!("{}", a);
    // }

    for a in array.iter() {
        println!("{}", a);
    }

    println!("len={}", array.len()); // ERROR
}