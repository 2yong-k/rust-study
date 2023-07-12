pub struct Node {
    data: isize,
    link: Option<Box<Node>>,
}

pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self { // 생성자
        Self{head: None}
    }

    pub fn unshift(&mut self, v:isize) {
        let new_node = Node{data: v, link: self.head.take()};
        self.head = Some(Box::new(new_node));
    }

    pub fn push(&mut self, v:isize) {
        let new_node = Node{data: v, link: None};
        match self.head {
            None => self.head = Some(Box::new(new_node)),
            Some(ref mut head) => {
                let mut p = head;
                loop {
                    match p.link {
                        None => { // 가장 끝 노드
                            p.link = Some(Box::new(new_node));
                            break;
                        },
                        Some(ref mut next) => p = next,
                    }
                }
            }
        }
    }

    pub fn get(&self, index: isize) -> Option<isize> {
        match self.head {
            None => return None, // 리스트가 비어있을 때
            Some(ref top) => {
                let mut p = top;
                let mut i = 0;
                loop {
                    if i == index {
                        return Some(p.data);
                    }
                    match p.link {
                        None => return None,
                        Some(ref link) => p = link,
                    }
                    i += 1;
                }
            }
        }
    }
}