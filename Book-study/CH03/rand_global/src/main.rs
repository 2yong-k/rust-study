use std::time::{SystemTime, UNIX_EPOCH};

// 전역 변수 선언시 static 필수
static mut SEED: u32 = 0;

// 가변 전역 변수를 사용하기 위해 unsafe 블록으로 감싸준다.
unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {
        let epoc = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap();
        SEED = epoc.as_millis() as u32;
    }
    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;
    return SEED % (end - start + 1) + start;
}

fn main() {
    // 100개의 난수 생성
    for _ in 0..100 {
        // unsafe한 함수를 사용하기 위해 unsafe 블록으로 감싸준다.
        unsafe {
            let v = rand_global(1, 6);
            println!("{}", v);
        }
    }
}