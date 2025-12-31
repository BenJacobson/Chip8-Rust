use crate::assembler_error::*;

#[test]
fn test_line_location_from_line_words() {
    let line = "Can you parse my text?";
    let words = vec!["parse", "text"];
    let Some(line_location) = LineLocation::try_from_line_words(line, &words, 1) else {
        panic!("LineLocation::try_from_line_words failed to parse");
    };

    assert_eq!(17, line_location.column);
    assert_eq!(4, line_location.length);
}
