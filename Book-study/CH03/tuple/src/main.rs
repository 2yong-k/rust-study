// 튜플로 정의
// fn main() {
//     let banana = ("바나나", 300);
//     let apple = ("사과", 200);

//     let total = banana.1 + apple.1;

//     print_tuple(&banana);
//     print_tuple(&apple);
//     println!("합계는 {}원입니다.", total);
// }

// fn print_tuple(item: &(&str, i64)) {
//     println!("{}를 {}원에 구입", item.0, item.1);
// }

// 구조체로 정의
struct Item(String, i64);

fn main() {
    let banana = Item("바나나".to_string(), 300);
    let apple = Item("사과".to_string(), 200);
    let mango = Item("망고".to_string(), 500);
    let items = vec![banana, apple, mango];
    let total = print_and_sum_items(&items);
    println!("합계는 {}원 입니다.", total);
}

fn print_tuple(item: &Item) {
    println!("{}를 {}원에 구입.", item.0, item.1);
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for it in items {
        print_tuple(&it);
        total += it.1;
    }
    total
}