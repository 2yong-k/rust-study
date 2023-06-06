// 그냥 계산할 시 오버플로 발생
// fn main() {
//     let v: u128 = 1234;
//     println!("{}", v.pow(5678));
// }

// Cargo.toml에 [dependencies] 크레이트 추가
use num_bigint::BigInt; // BigInt 사용을 위해

fn main() {
    let v = BigInt::from(1234);
    println!("{}", v.pow(5678));
}