use chip8_assembler::assemble;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Expected command line argument of path to chip8 assembly file.");
        return;
    }

    let assembly_filepath = &args[1];
    let output_filepath = args
        .iter()
        .find_map(|arg| {
            if arg.starts_with("-o=") {
                Some(arg[3..].to_string())
            } else {
                None
            }
        })
        .expect("Expected output file path arg: -o={FILE_PATH}");
    if !fs::metadata(&output_filepath).is_err() {
        println!("Output file already exists: {}", output_filepath);
        return;
    }

    let assembly_result = fs::read_to_string(assembly_filepath);
    if let Err(e) = assembly_result {
        println!("Error reading assembly file: {}", e);
        return;
    }
    let assembly_text = assembly_result.unwrap();
    let program_result = assemble(&assembly_text, MEM_ADDR_START, MEM_ADDR_MAX);

    if let Err(ref errors) = program_result {
        println!("Error(s) assembling {}:", assembly_filepath);
        for error in errors {
            println!("  {:?}", error);
        }
        return;
    }

    let program = program_result.unwrap();
    let write_result = fs::write(output_filepath, program);
    if let Err(e) = write_result {
        println!("Error writing to output file: {}", e);
        return;
    }
}

const MEM_ADDR_START: u16 = 0x200;
const MEM_ADDR_MAX: u16 = 4096;
