pub struct Display<'a> {
    pub data: &'a [u8],
    pub width: usize,
    pub height: usize,
}

impl<'a> Display<'a> {
    pub fn new(data: &'a [u8], width: usize, height: usize) -> Self {
        assert_eq!((width * height + 7) / 8, data.len());
        Self {
            data,
            width,
            height,
        }
    }

    pub fn get_pixel(&self, i: usize, j: usize) -> bool {
        if !(j < self.width && i < self.height) {
            return false;
        }

        let pixel = i * self.width + j;
        let byte = pixel >> 3;
        let bit = 7 - (pixel & 0x7);
        return self.data[byte] & (1 << bit) != 0;
    }
}
