#[test]
fn test_string_collect() {
    let s = "Man hat tan";
    let city: String = s.chars().filter(|c| !c.is_whitespace()).collect();

    assert_eq!(city, "Manhattan");
}