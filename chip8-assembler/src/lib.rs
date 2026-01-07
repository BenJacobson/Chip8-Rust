mod assembler_error;
mod parse;
mod statement;

use assembler_error::*;
use parse::*;
use statement::*;

use std::collections::HashMap;

pub fn assemble(
    text: &str,
    mem_addr_start: u16,
    mem_addr_max: u16,
) -> Result<Vec<u8>, Vec<AssemblerError>> {
    let mut statements: Vec<Statement> = text
        .lines()
        .enumerate()
        .flat_map(|(line_num, line)| get_partial_statement(line, line_num as u16))
        .collect();

    let mut next_mem_addr: u16 = mem_addr_start;
    for statement in statements.iter_mut() {
        if next_mem_addr >= mem_addr_max {
            return Err(vec![AssemblerError::new_no_options(
                format!("Memory exceeds available space (0x{:x})", mem_addr_max),
                Location::new_line_num(statement.line_num),
            )]);
        }
        statement.mem_addr = next_mem_addr;
        next_mem_addr += statement.size;
    }

    let mut labels: HashMap<String, u16> = HashMap::new();
    for statement in statements.iter() {
        let StatementType::Label { .. } = statement.statement_type else {
            continue;
        };
        let name = parse_label(statement.line.as_str(), statement.line_num)
            .map_err(|err| vec![err])?
            .to_string();
        if name.is_empty() {
            return Err(vec![AssemblerError::new_no_options(
                "Cannot use an empty label.".to_string(),
                Location::new(None, statement.line_num),
            )]);
        }
        if labels.contains_key(name.as_str()) {
            return Err(vec![AssemblerError::new_no_options(
                format!("Label '{}' already used.", name),
                Location::new(None, statement.line_num),
            )]);
        }
        labels.insert(name, statement.mem_addr);
    }

    for statement in statements.iter_mut() {
        match statement.statement_type {
            StatementType::Bytes { ref mut data } => {
                *data = parse_bytes(statement.line.as_str())?;
            }
            StatementType::Instruction {
                ref mut instruction,
            } => {
                *instruction = parse_instruction(
                    statement.line.as_str(),
                    statement.line_num,
                    &labels,
                    mem_addr_max,
                )
                .map_err(|err| vec![err])?
            }
            StatementType::Label { ref mut name } => {
                *name = parse_label(&statement.line, statement.line_num)
                    .map_err(|err| vec![err])?
                    .to_string();
            }
        };
    }

    let bytes: Vec<u8> = statements
        .iter()
        .map(|statement| statement.bytes())
        .flatten()
        .collect();
    Ok(bytes)
}

#[cfg(test)]
mod assembler_error_test;

#[cfg(test)]
mod lib_test;

#[cfg(test)]
mod parse_test;

#[cfg(test)]
mod statement_test;
