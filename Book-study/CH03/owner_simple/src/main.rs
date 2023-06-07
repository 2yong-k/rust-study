// 소유권 이전 확인
// fn main() {
//     let g1 = String::from("온화한 마음은 몸에 좋다.");
//     let g2 = g1;    

//     // ERROR: 소유권 이전.
//     // println!("{}", g1);

//     // 정상. 소유권이 이전되어서.
//     println!("{}", g2);
// }

// 블록 범위까지만 소유권이 유효한지 확인
// fn main() {
//     {
//         let s1 = String::from("재능은 한계가 있지만 노력엔 한계가 없다");
//         println!("{}", s1);
//     }
//     // ERROR: 블록 범위 밖
//     // println!("{}", s1);
// }

// 기본 타입에서 소유권이 이동하지 않는 것을 확인
// fn main() {
//     let g1 = 30;
//     let g2 = g1;
//     println!("{}", g1);
//     println!("{}", g2);
// }

// 소유권 이동 말고, 데이터를 복제하는 방법
fn main() {
    let g1 = String::from("온화한 마음은 몸에 좋다.");
    let g2 = g1.clone();
    println!("{}", g1);
    println!("{}", g2);
}