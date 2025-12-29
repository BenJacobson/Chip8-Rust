use chip8_instructions::Instruction as Chip8Instruction;
use itertools::Itertools;

const ADDR_SPACE: u16 = 4096;

pub struct AssemblerError {
    src_location: Option<Location>,
    message: String,
}

pub struct Location {
    line_num: u32,
    char_start: u32,
    char_end: u32,
}

pub fn assemble(text: &str, mem_addr_start: u16) -> Result<Vec<u8>, Vec<AssemblerError>> {
    match text.len() {
        l if l > 0 => Ok(Vec::new()),
        _ => Err(vec![AssemblerError {
            src_location: None,
            message: "".to_string(),
        }]),
    }
}

enum Statement<'a> {
    Label { name: &'a str, offset: u16 },
    Instruction(Chip8Instruction),
}

fn parse_lines<'a>(text: &'a str) -> Result<Vec<Statement<'a>>, Vec<AssemblerError>> {
    let (oks, errs): (Vec<Statement>, Vec<AssemblerError>) = text
        .lines()
        .enumerate()
        .flat_map(|(line_num, line)| parse_line(line, line_num as u32))
        .partition_result();
    if errs.len() > 0 { Err(errs) } else { Ok(oks) }
}

fn parse_line<'a>(line: &'a str, line_num: u32) -> Option<Result<Statement<'a>, AssemblerError>> {
    let words: Vec<_> = line.split_whitespace().collect();
    if words.is_empty() {
        return None;
    }
    if words.len() == 1 && words[0].ends_with(":") {
        let label = words[0].chars().dropping_back(1).as_str();
        if label.is_empty() {
            return Some(Err(AssemblerError {
                src_location: None,
                message: "Expected label".to_string(),
            }));
        }
        return Some(Ok(Statement::Label {
            name: label,
            offset: 0,
        }));
    }
    None
}
