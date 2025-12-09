use super::display::*;

#[test]
fn test_get_pixel() {
    let data: [u8; 8] = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8];
    let display = Display::new(&data, 8, 8);
    for i in 0..8 {
        for j in 0..8 {
            let pixel = data[i] & (1 << (7-j)) != 0;
            assert_eq!(pixel, display.get_pixel(i, j));
        }
    }
}
