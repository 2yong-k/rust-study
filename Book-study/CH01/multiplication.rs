fn multiplication(a: i64, b: i64) -> i64 {
    // return a*b; 또는 a*b 이런 식으로 return과 ;을 생략할 수 있다.
    a*b
}

fn main() {
    let ex1 = multiplication(3, 5);
    println!("3*5={}", ex1);

    let ex2 = multiplication(8, 4);
    println!("8*4={}", ex2);
}