// 01. 함수 호출로 인한 소유권 이동
// fn main() {
//     let g1 = String::from("실수할 줄 아는 사람이 아름답다.");
//     show_message(g1);
//     // ERROR: 함수 호출로 인해 소유권 이동이 일어남
//     // println!("{}", g1);
// }
// fn show_message(message: String) {
//     println!("{}", message);
// }

// 02. 함수 호출로 인한 소유권 이동과 다시 돌려주기
// fn main() {
//     let mut g1 = String::from("실수할 줄 아는 사람이 아름답다.");
//     g1 = show_message(g1);
//     println!("{}", g1);
// }
// fn show_message(message: String) -> String {
//     println!("{}", message);
//     return message;
// }

// 03. 참조 값을 전달하여 함수에 소유권을 이전하지 않는 방법
// fn main() {
//     let g1 = String::from("실수할 줄 아는 사람이 아름답다.");
//     show_message(&g1);
//     println!("{}", g1);
// }
// fn show_message(message: &String) {
//     println!("{}", message);
// }

// 04. 참조자를 반환받아 사용
// fn main() {
//     let m = gen_message();
//     println!("{}", m);
// }
// fn gen_message() -> String {
//     let msg = String::from("실수할 줄 아는 사람이 아름답다");
//     return msg;
// }

// 05. 가변 참조자를 인수로 사용
// fn main() {
//     let mut msg = String::from("건강한 신체에 건강한 정신이 깃든다.");
//     println!("{}", msg);
//     add_quote(&mut msg);
//     println!("{}", msg);
// }
// fn add_quote(msg: &mut String) {
//     msg.insert(0, '"');
//     msg.push('"');
// }

// 06. 함수를 호출해서 인수를 변경
fn main() {
    let mut msg1 = String::from("TEST");
    let msg2 = msg1.clone();
    change_msg(&mut msg1);
    println!("{}", msg1);
    println!("{}", msg2);
}

fn change_msg(message: &mut String) {
    *message = String::from("TEST2");
}