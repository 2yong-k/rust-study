use image::{Rgb, ImageBuffer};
fn main() {
    let white = Rgb::<u8>([255, 255, 255]);
    let red = Rgb::<u8>([255, 90, 90]);

    let w = 64;
    let draw = |x, y| { // 클로저
        let (xi, yi) = (x / w, y / w);
        match (xi % 2, yi % 2) {
            (0, 0) => white,
            (1, 0) => red,
            (0, 1) => red,
            (1, 1) => white,
            (_, _) => panic!("error"),
        }
    };
    let img = ImageBuffer::from_fn(512, 512, draw);
    img.save("checkerboard.png").unwrap();
}