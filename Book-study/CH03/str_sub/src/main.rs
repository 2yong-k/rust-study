fn main() {
    hex_dump("성공하는 사람은 송곳처럼 어느 한 점을 향하여 말한다.");
    str_slice("지혜는 무기보다 가치가 있다.");
    str_substr("🥺🐇🧸🥇🤣");
    str_substr2("지혜는 무기보다 가치가 있다.");
}

fn hex_dump(s: &str) {
    for (i, c) in s.bytes().enumerate() {
        // 주소를 표시
        if i % 16 == 0 {
            print!("{:08x}|", i);
        }
        // 4자리씩 끊어 문자를 표시
        if i % 4 == 3 {
            print!("{:02x}|", c);
        } else {
            print!("{:02x} ", c);
        }
        // 16바이트마다 줄바꿈
        if i % 16 == 15 {
            println!("");
        }
    }
    println!("");
}

fn str_slice(s: &str) {
    println!("앞 2글자: {}", &s[0..6]);
    println!("4~5번째 글자: {}", &s[10..16]);
}

fn str_substr(s: &str) {
    let mut sub1 = String::new();
    for (i, c) in s.chars().enumerate() {
        if i < 2 {
            sub1.push(c);
            continue;
        }
        break;
    }
    println!("앞 2글자: {}", sub1);

    let mut sub2 = String::new();
    for (i, c) in s.chars().enumerate() {
        if 3 <= i && i <= 4 {
            sub2.push(c);
        }
    }
    println!("4~5번째 글자: {}", sub2);
}

fn str_substr2(s: &str) {
    let sub1: String = s.chars().take(2).collect();
    println!("앞 2글자: {}", sub1);

    let pr_chars: Vec<char> = s.chars().collect();
    let sub_chars = &pr_chars[4..6];
    let sub2: String = sub_chars.into_iter().collect();
    println!("4~5번째 글자: {}", sub2);
}