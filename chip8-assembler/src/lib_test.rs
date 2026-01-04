use crate::*;

const DUPLICATE_LABELS_SRC: &str = "
label1:
label2:
label1:
";

#[test]
fn test_duplicate_labels() {
    let result = assemble(DUPLICATE_LABELS_SRC, 0);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    let duplicate_label_error = errors.first().unwrap();
    assert_eq!("Label already used.", duplicate_label_error.message);
}

const COMMENTS_SRC: &str = "
# comment 1
  # comment 2
; semicolon test
// double slash test
@ doc comment test 
";

#[test]
fn test_comments() {
    let result = assemble(COMMENTS_SRC, 0);
    assert!(result.is_ok());
    assert_eq!(0, result.unwrap().len());
}
