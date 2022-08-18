#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/form_maker_tests.rs");
}
