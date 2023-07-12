
// pub struct Node {
//     data: i64,
//     link: Option<Box<Node>>,
// }

// fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
//     Some(Box::new(Node {
//         data: v,
//         link: link,
//     }))
// }

// fn main() {
//     let c = node(10, node(20, node(30, None))).unwrap();

//     let mut p = &c;
//     loop {
//         println!("{}", p.data);
//         match p.link {
//             None => break,
//             Some(ref link) => p = &link,
//         }
//     }
// }

// enum을 이용하여 구현
enum Node {
    Empty,
    Cons(i64, Box<Node>),
}

use Node::{Empty, Cons};
fn node(v: i64, link: Box<Node>) -> Box<Node> {
    Box::new(Cons(v, link))
}

fn main() {
    let c = node(10, node(20, node(30, Box::new(Empty))));

    let mut ptr: &Box<Node> = &c;
    loop {
        let cur_node: &Node = &**ptr; 
        match cur_node {
            Empty => break,
            Cons(v, link) => {
                println!("{}", v);
                ptr = &link;
            }
        }
    }
}