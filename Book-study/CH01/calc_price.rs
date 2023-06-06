/*
    어떤 컴퓨터 업체의 PC 가격은 98만원이다.
    A 쇼핑몰에서는 배송비가 12,000원이고, 20% 할인된 가격으로 판매하고 있다.
    B 쇼핑몰에서는 배송비가 무료지만, 10%할인된 가격으로 팔고 있다.
    두 쇼핑몰 중 어느 쇼핑몰이 더 싼가?
 */
fn main() {
    let pc_price = 980000.0;
    let a_ship_fee = 12000.0;
    let a_rate = 0.8;
    let b_ship_fee = 0.0;
    let b_rate = 0.9;

    println!("A 쇼핑몰={}원", pc_price * a_rate + a_ship_fee);
    println!("B 쇼핑몰={}원", pc_price * b_rate + b_ship_fee);
}