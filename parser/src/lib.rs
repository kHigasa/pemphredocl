#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub rustreeem);

#[test]
fn rustreeem() {
    assert!(rustreeem::NumParser::new().parse("22").is_ok());
}

