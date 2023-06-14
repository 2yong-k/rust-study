fn main() {
    str_find("제주도의 특산품 중 귤은 겨울에 많이 먹을 수 있다.");
    str_find_upper(&format!("{}{}", "There is more happiness in giving ", "than there is in receiving."));
}

fn str_find(s: &str) {
    match s.find("귤") {
        Some(i) => println!("귤 = {}B", i),
        None => println!("'귤'이라는 단어는 없습니다."),
    };

    match s.find("바나나") {
        Some(i) => println!("바나나 = {}B", i),
        None => println!("'바나나'라는 단어는 없습니다."),
    };
}

fn str_find_upper(s: &str) {
    let res = s.find(|c:char| c.to_ascii_uppercase() == 'S');
    match res {
        Some(i) => println!("S={}B", i),
        None => println!("None"),
    };
}