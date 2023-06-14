fn main() {
    let ss: &str = "베풀면 반드시 돌아온다.";

    let so1: String = String::from(ss);
    let so2: String = ss.to_string();

    let ss2: &str = &so1;
    let ss3: &str = so1.as_str();

    println!("{}\n{}\n{}\n{}", so1, so2, ss2, ss3); // 변환된 문자열 출력
    println!("{:p}\n{:p}", ss2, ss3);   // 포인터 주소 표시

    for c in ss.bytes() {
        print!("{:2x} ", c);
    }
    println!("\n바이트 = {}B", ss.len());

     let pr = "구슬이 서 말이라도 꿰어야 보배";
    for c in pr.chars() {
        print!("[{}]", c);
    }
    println!("\n글자 수 = {}자", pr.chars().count());

    // Vec<char>로 변환해 처리
    let pr_chars: Vec<char> = pr.chars().collect();
    println!("Vec<char> : {:?}", pr_chars);
    for c in pr_chars.iter() {
        print!("({})", c);
    }
    println!("\n글자 수 = {}자", pr_chars.len());
}