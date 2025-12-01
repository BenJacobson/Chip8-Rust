use super::key::*;

#[test]
fn test_convert_key_to_keys() {
    assert_eq!(key_number_to_keys(0x0), Key::_0 as u16);
    assert_eq!(key_number_to_keys(0x1), Key::_1 as u16);
    assert_eq!(key_number_to_keys(0x2), Key::_2 as u16);
    assert_eq!(key_number_to_keys(0x3), Key::_3 as u16);
    assert_eq!(key_number_to_keys(0x4), Key::_4 as u16);
    assert_eq!(key_number_to_keys(0x5), Key::_5 as u16);
    assert_eq!(key_number_to_keys(0x6), Key::_6 as u16);
    assert_eq!(key_number_to_keys(0x7), Key::_7 as u16);
    assert_eq!(key_number_to_keys(0x8), Key::_8 as u16);
    assert_eq!(key_number_to_keys(0x9), Key::_9 as u16);
    assert_eq!(key_number_to_keys(0xA), Key::_A as u16);
    assert_eq!(key_number_to_keys(0xB), Key::_B as u16);
    assert_eq!(key_number_to_keys(0xC), Key::_C as u16);
    assert_eq!(key_number_to_keys(0xD), Key::_D as u16);
    assert_eq!(key_number_to_keys(0xE), Key::_E as u16);
    assert_eq!(key_number_to_keys(0xF), Key::_F as u16);
}

#[test]
fn test_stable_conversion() {
    for i in 0..16 {
        assert!(keys_has_any_key(
            key_number_to_keys(i),
            key_number_to_keys(i)
        ));
        assert_eq!(i, keys_to_key_number(key_number_to_keys(i)));
    }
}
