use rand::seq::SliceRandom;

fn main() {
    let mut nums = [0; 75];
    for i in 1..=75 { nums[i-1] = i; }  // 1~75가 써있는 75장의 카드 만들기

    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng); // 배열 섞기
    
    // 5*5 빙고판에 카드 두기
    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 { // 와일드 카드
                print!("  *,"); // 직접 숫자 배열에 넣을 수 없기 때문에 따로 출력
            } else {
                print!("{:3},", nums[i]);
            }
        }
        println!("");
    }
}