#[test]
pub fn pass() {
    // This tells macrotest to expand every .rs file in the `tests/expand` folder
    macrotest::expand("tests/expand/*.rs");
}