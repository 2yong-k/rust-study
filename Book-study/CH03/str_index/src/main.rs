fn main() {
    let s = "안녕하세요";
    let ch = s.chars().nth(0).unwrap();
    println!("{}", ch);
    let ch2 = &s[..3];
    println!("{}", ch2);

    let s2 = "abcdefg";
    println!("{}", &s2[0..1]);
}