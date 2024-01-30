
pub use astray_core::*;
pub use astray_macro::*;

fn a(a: i32) {
    assert_eq!(a, 2);
    let v: Vec<String> = ["ha", "hb"].iter().map(|s|s.to_string()).collect();
    b(&v);
}

fn b (a: &[String]) {

}
