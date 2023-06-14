fn main() {
    echo("웅변은 은이요");
    echo("침묵은 금이다");

    // ERROR: &s는 main 함수에서만 살아있을 수 있기에 static보다 짧은 라이프타임이기에 에러 발생.
    // let s = String::from("TEST");
    // echo(&s);

    let s2: &str = "TEST";
    echo(s2);
}

fn echo(s: &'static str) {
    println!("{}", s);
}