use std::io;
fn main() {
    // 계산용 스택
    let mut stack: Vec<f64> = vec![];

    // 수식 얻기
    println!("RPN:");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("입력 에러");

    // 배열로 변환한 뒤, 반복문을 통해 계산
    let tokens = s.split_whitespace();
    for tok in tokens {
        let t = tok.trim();
        match t.parse::<f64>() {
            Ok(v) => { stack.push(v); continue; },
            Err(_) => 0.0,
        };
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        match t {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "*" => stack.push(a * b),
            "/" => stack.push(a / b),
            _ => panic!("계산이 불가능한 연산자 : {}",t),
        }
    }

    // 결과값 출력
    println!("{}", stack.pop().unwrap());
}