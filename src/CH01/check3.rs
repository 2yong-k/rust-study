fn main() {
    // 3의 배수와 3이 포함된 숫자는 A로 표시
    for i in 1..51 {
        if i % 3 == 0 || i % 10 == 3 {
            println!{"A"};
            continue;
        }
        if i >= 30 && i <= 39 {
            println!{"A"};
            continue;
        }
        println!{"{}", i};
    }
}