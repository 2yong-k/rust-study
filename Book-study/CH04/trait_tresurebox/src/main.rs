// trait TreasureBox {
//     fn open(&self, key_no: i32) -> bool;
//     fn check(&self);
// }

// struct JewelryBox {
//     price: i32,  // 몇 골드가 있는가
//     key_no: i32, // 몇 번째 열쇠가 있어야 열리는가
// }
// impl TreasureBox for JewelryBox {
//     fn open(&self, key_no: i32) -> bool {
//         self.key_no == key_no
//     }
//     fn check(&self) {
//         println!("보석 상자였다. {} 골드 입수.", self.price);
//     }
// }

// struct TrapBox {
//     damage: i32,
// }
// impl TreasureBox for TrapBox {
//     fn open(&self, _key: i32) -> bool {
//         return true; // 어떤 키를 가지고 있어도 열림
//     }
//     fn check(&self) {
//         println!("함정이었다. HP가 {} 감소했다.", self.damage);
//     }
// }

// fn open_box(tbox: &impl TreasureBox, key_no: i32) {
//     if !tbox.open(key_no) {
//         println!("열쇠가 맞지 않아 상자가 열리지 않는다.");
//         return;
//     }
//     tbox.check();
// }

// fn main() {
//     let box1 = JewelryBox {
//         price: 30,
//         key_no: 1,
//     };
//     let box2 = TrapBox { damage: 3 };
//     let box3 = JewelryBox {
//         price: 20,
//         key_no: 2,
//     };

//     let my_key = 2;
//     open_box(&box1, my_key);
//     open_box(&box2, my_key);
//     open_box(&box3, my_key);
// }

trait TreasureBox {
    // 기본 메서드
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}

struct JewelryBox {
    price: i32,
    key_no: i32,
}
impl TreasureBox for JewelryBox {
    fn check(&self) {
        println!("보석 상자였다. {} 골드 입수.", self.price);
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

struct EmptyBox {
    key_no: i32,
}
impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("상자는 비어있다.");
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("열쇠가 맞지 않아 상자가 열리지 않는다.");
        return;
    }
    tbox.check();
}

fn main() {
    let box1 = JewelryBox {
        price: 30,
        key_no: 1,
    };
    let box2 = EmptyBox { key_no: 1 };
    let box3 = JewelryBox {
        price: 50,
        key_no: 2,
    };

    open_box(&box1, 1);
    open_box(&box2, 1);
    open_box(&box3, 1);
}