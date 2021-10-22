use super::*;

#[test]
fn test_heading_1_implementation() {
    let a = H1::new("this is a heading test");
    assert_eq!("<h1>this is a heading test</h1>", a.to_html());
}