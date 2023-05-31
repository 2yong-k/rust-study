use rand::seq::SliceRandom;

fn main() {
    let mut nums = vec![];
    for i in 1..=75 { nums.push(i); }   // 따로 push를 이용해서 넣어준다.
    
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    for i in 0..25 {
        if i == 12 {
            print!("  *,");  // 와일드 카드
        } else {
            print!("{:3},", nums[i]);
        }
        if i % 5 == 4 { println!(""); }
    }
}