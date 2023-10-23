use crate::OnceArc;

#[test]
fn test_abc() {
    let once = OnceArc::new(123);
    println!("{once:?}");
}