use crate::parse::*;

#[test]
fn test_error_parsing_number() {
    let Err(errors) = parse_lines("jp abc") else {
        panic!("Parsing statements failed to fail.");
    };

    assert_eq!(1, errors.len());
    let Some(error) = errors.first() else {
        panic!();
    };

    assert!(error.message.find("Failed to parse number").is_some());
}
