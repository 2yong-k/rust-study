use wikibooks_rpn_calc_test_ver_zero_one_one as rpn_calc;

fn main() {
    let src = "2 3 4 * +".to_string();
    let ans = rpn_calc::eval(src).unwrap();
    println!("{}", ans);
}