use super::key::*;

#[test]
fn test_single_keys_to_key_number() {
    assert_eq!(0x0, keys_to_key_number(Key::_0 as u16));
    assert_eq!(0x1, keys_to_key_number(Key::_1 as u16));
    assert_eq!(0x2, keys_to_key_number(Key::_2 as u16));
    assert_eq!(0x3, keys_to_key_number(Key::_3 as u16));
    assert_eq!(0x4, keys_to_key_number(Key::_4 as u16));
    assert_eq!(0x5, keys_to_key_number(Key::_5 as u16));
    assert_eq!(0x6, keys_to_key_number(Key::_6 as u16));
    assert_eq!(0x7, keys_to_key_number(Key::_7 as u16));
    assert_eq!(0x8, keys_to_key_number(Key::_8 as u16));
    assert_eq!(0x9, keys_to_key_number(Key::_9 as u16));
    assert_eq!(0xA, keys_to_key_number(Key::_A as u16));
    assert_eq!(0xB, keys_to_key_number(Key::_B as u16));
    assert_eq!(0xC, keys_to_key_number(Key::_C as u16));
    assert_eq!(0xD, keys_to_key_number(Key::_D as u16));
    assert_eq!(0xE, keys_to_key_number(Key::_E as u16));
    assert_eq!(0xF, keys_to_key_number(Key::_F as u16));
}

#[test]
fn test_multiple_keys_to_key_number() {
    assert_eq!(0xA, keys_to_key_number((Key::_A as u16) | (Key::_B as u16)));
}
