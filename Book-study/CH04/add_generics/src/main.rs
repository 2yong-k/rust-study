// 기존 방법: 제너릭을 사용하지 않으면 타입별로 메서드를 만들었어야 한다.
// fn add_i32(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn add_f32(a: f32, b: f32) -> f32 {
//     a + b
// }

// 제네릭을 이용해 add 정의
// fn add <T: std::ops::Add<Output=T>> (a:T, b:T) -> T {
//     a + b
// }


// 제네릭에 where 사용
fn add <T> (a:T, b:T) -> T 
    where T: std::ops::Add<Output=T>
{
    a + b
}

fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
    println!("{}", add::<i32>(10, 25)); // 타입을 명시할 때
    // println!("{}", add('a', 'a')); --- char 타입에 대한 것은 덧셈 불가. 미구현 에러 발생
}