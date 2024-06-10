use asmlib::instruction::*;
use std::fs::*;
use std::io;
use std::io::BufRead;
use std::path::*;
use blex::Lexer;
use blas::asm::ASM;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn blasm_to_instructions(filename: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::with_capacity(20);

    let read_attempt = read_to_string(filename);

    if let Ok(lines_of_code) = read_attempt {
        let lexer = Lexer::new(&lines_of_code);
        let asm = ASM::new(&lines_of_code, &lexer);

        for pseudo_instruction in asm {
            if let Ok(pseudo_instruction) = pseudo_instruction {
                let instruction: Instruction = pseudo_instruction.into();

                instructions.push(instruction);
            } else {
                println!("[blasm_to_instructions] fail to convert pseudo_instruction");
            }
        }

        return instructions;
    }
    panic!("[blasm_to_instructions] fail to read file");
}

pub fn binary_to_instructions(filename: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::with_capacity(20);
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(inst_binary_str) = line {
                let instruction: u32 = u32::from_str_radix(&inst_binary_str, 2).unwrap();
                instructions.push(decode_instruction(instruction));
            }
        }
    }
    instructions
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn decode_instruction_iinstruction() {
        let instruction = binary_to_instructions("examples/binary1");
        assert_eq!(
            encode_instruction(instruction[0]) as u32,
            0b0110000001000000001
        );
    }
}
