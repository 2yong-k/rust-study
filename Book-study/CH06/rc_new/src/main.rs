// use std::rc::Rc;
// fn main() {
//     let a_rc = Rc::new(1000);
//     {
//         let b_rc = Rc::clone(&a_rc);
//         println!("{}", b_rc);
//         println!("참조 카운트 = {}", Rc::strong_count(&a_rc));
//     }
//     println!("{}", a_rc);
//     println!("참조 카운트 = {}", Rc::strong_count(&a_rc));
// }

// 힙 영역 공유 가능 + 값 변경 가능
use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let a = Rc::new(RefCell::new(1000));
    let b = Rc::clone(&a);
    *b.borrow_mut() += 100;
    println!("{}", a.borrow());
}