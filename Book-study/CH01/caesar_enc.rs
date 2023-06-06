fn encrypt(text: &str, shift: i16) -> String {
    // A와 Z의 문자코드를 i16타입으로 받기
    // 여기서 유니코드까지 지원하고 싶다면 정수 타입을 u32로 지정.
    // 러스트는 문자(char)타입을 표현하기 위해 4바이트(32비트)를 사용
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;

    // 결과값이 들어갈 result 변수 선언
    let mut result = String::new();

    // text를 받아 한 글자씩 치환 처리
    for ch in text.chars() {
        // 한 글자씩 문자 코드로 변환
        let mut code = ch as i16;

        // A~Z 사이에 있는 문자를 shift만큼 옮겨서 다시 넣어주기
        if code_a <= code && code <= code_z {
            code = (code - code_a + shift + 26) % 26 + code_a;
        }

        // 문자 코드로 되어있는 code를 다시 문자로 바꿔서 결과값에 push
        // only 'u8' can be cast as 'char', not 'i16'
        result.push((code as u8) as char);
    }

    // 결과값 리턴
    return result;
}

fn main() {
    let enc = encrypt("I LOVE RUST.", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}