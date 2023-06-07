// 배열 슬라이스 실행
// fn main() {
//     let a = [0, 1, 2, 3, 4, 5];
//     let a_slice = &a[0..3];
//     println!("{:?}", a_slice); // [0, 1, 2]
//     println!("{:?}", &a[3..5]); // [3, 4]
//     println!("{:?}", &a[4..6]); // [4, 5]
// }

// 함수를 이용해 값 사용해보기
fn main() {
    let a = [1,2,3,4,5,6,7,8,9,10];
    println!("a={}", sum_slice(&a[..]));    // [..] 전체 슬라이스 이용
    println!("{:?}", a);

    let b = vec![1,2,3,4,5,6,7,8,9,10];
    println!("b={}", sum_slice(&b[..]));
    println!("{:?}", b);
}

fn sum_slice(items: &[i64]) -> i64 {
    let mut total = 0;
    for i in items {
        total += i;
    }
    total
}