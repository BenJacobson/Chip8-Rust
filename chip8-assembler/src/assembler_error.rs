#[derive(Debug)]
pub struct AssemblerError {
    pub message: String,
    pub src_location: Option<Location>,
}

impl AssemblerError {
    pub fn new(message: String, src_location: Option<Location>) -> Self {
        Self {
            message,
            src_location,
        }
    }

    pub fn new_no_options(message: String, src_location: Location) -> Self {
        Self::new(message, Some(src_location))
    }

    pub fn new_message(message: String) -> Self {
        Self::new(message, None)
    }
}

#[derive(Debug)]
pub struct Location {
    pub line_location: Option<LineLocation>,
    pub line_num: u16,
}

impl Location {
    pub fn new(line_location: Option<LineLocation>, line_num: u16) -> Self {
        Self {
            line_location,
            line_num,
        }
    }

    pub fn new_no_options(line_location: LineLocation, line_num: u16) -> Self {
        Self::new(Some(line_location), line_num)
    }

    pub fn new_line_num(line_num: u16) -> Self {
        Self::new(None, line_num)
    }
}

#[derive(Debug)]
pub struct LineLocation {
    pub column: u32,
    pub length: u32,
}

impl LineLocation {
    pub fn new(column: u32, length: u32) -> Self {
        Self { column, length }
    }

    pub fn try_from_line_words(
        line: &str,
        words: &Vec<&str>,
        word_index: usize,
    ) -> Option<LineLocation> {
        if word_index >= words.len() {
            return None;
        }

        let mut start = 0;
        for word in words[0..=word_index].iter() {
            let Some(i) = line[start..].find(word) else {
                return None;
            };
            start += i + word.len();
        }

        let word_len = words[word_index].len();
        Some(LineLocation::new(
            (start - word_len) as u32,
            word_len as u32,
        ))
    }
}
