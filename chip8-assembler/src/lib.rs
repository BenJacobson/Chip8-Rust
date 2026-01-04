mod assembler_error;
mod parse;
mod statement;

use assembler_error::AssemblerError;
use parse::parse_lines;
use statement::Statement;

use std::collections::HashMap;

const ADDR_SPACE: u16 = 4096;

pub fn assemble(text: &str, mem_addr_start: u16) -> Result<Vec<u8>, Vec<AssemblerError>> {
    let mut statements = parse_lines(text)?;
    let mut next_mem_addr = mem_addr_start;
    for statement in statements.iter_mut() {
        statement.set_mem_addr(next_mem_addr);
        if let Statement::Instruction { .. } = statement {
            next_mem_addr += 0x2;
        }
    }
    let mut labels: HashMap<&str, u16> = HashMap::new();
    for statement in statements.iter() {
        if let Statement::Label { name, mem_addr } = statement {
            if let Some(_) = labels.get(name) {
                return Err(vec![AssemblerError::new_message(
                    "Label already used.".to_string(),
                )]);
            } else {
                labels.insert(name, *mem_addr);
            }
        }
    }
    Ok(Vec::new())
}

#[cfg(test)]
mod assembler_error_test;

#[cfg(test)]
mod lib_test;

#[cfg(test)]
mod parse_test;

#[cfg(test)]
mod statement_test;
