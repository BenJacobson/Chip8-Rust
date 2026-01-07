use crate::*;

const DUPLICATE_LABELS_SRC: &str = "
label1:
label2:
label1:
";

#[test]
fn test_duplicate_labels() {
    let result = assemble(DUPLICATE_LABELS_SRC, 0, 0x100);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    let duplicate_label_error = errors.first().unwrap();
    assert_eq!("Label 'label1:' already used.", duplicate_label_error.message);
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
    let result = assemble(COMMENTS_SRC, 0, 0x100);
    assert!(result.is_ok());
    assert_eq!(0, result.unwrap().len());
}

const MEM_SPACE_EXCEEDED_SRC: &str = "
0x00 0x00
0x00 0x00
0x00 0x00
0x00 0x00
";

#[test]
fn test_mem_space_exceeded() {
    let result = assemble(MEM_SPACE_EXCEEDED_SRC, 0, 0x2);
    assert!(result.is_err());
    
    let errors = result.unwrap_err();
    let duplicate_label_error = errors.first().unwrap();
    assert_eq!("Memory exceeds available space (0x2)", duplicate_label_error.message);
}

const BASIC_PROGRAM_SRC: &str = "
  LD V1 10
  JP 0x4
  0x01 0x02 0x03 0x04
";

#[test]
fn test_basic_program() {
    let result = assemble(BASIC_PROGRAM_SRC, 0, 0x20);
    assert!(result.is_ok(), "Expected Ok, but got {:?}", result);
    
    let bytes = result.unwrap();
    assert_eq!(vec![97, 10, 16, 4, 1, 2, 3, 4], bytes);
}

// const PROGRAM_WITH_LABELS_SRC: &str = "
// start:
//   LD V1 10
//   JP start
// some_data:
//   0x01 0x02 0x03 0x04
// ";

// #[test]
// fn test_program_with_labels() {
//     let result = assemble(PROGRAM_WITH_LABELS_SRC, 0, 0x20);
//     assert!(result.is_ok(), "Expected Ok, but got {:?}", result);
    
//     let bytes = result.unwrap();
//     assert_eq!(vec![0x00], bytes);
// }
