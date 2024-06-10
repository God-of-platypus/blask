use std::{
    error::Error,
    fs::OpenOptions,
    io::{Read, Write},
};

use asmlib::instruction::{encode_instruction, Instruction};
use blas::ASM;
use blex::Lexer;
use clap::Parser;

/// Blask Assembler program to assemble your files to binary.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about= None)]
struct Args {
    /// Name of the file to assemble.
    input_file: String,

    /// Name of the binary file outputed.
    #[arg(short = 'o', long = "output", default_value_t = String::from("output.bin"))]
    output_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Open input file
    let mut input = OpenOptions::new().read(true).open(args.input_file)?;

    // Read all the file in a String
    let mut text = String::new();
    input.read_to_string(&mut text)?;

    // Create the Lexer
    let lexer = Lexer::new(&text);

    // Create the ASM
    let asm = ASM::new(&text, &lexer);

    // Open output file
    let mut output = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(args.output_file)?;

    for instr in asm {
        let instr = instr.unwrap();

        // Convert the PseudoInstruction to Binary

        let instr: Instruction = instr.into();

        // Write binary to output file
        let bytes = encode_instruction(instr);

        output.write_all(&bytes.to_le_bytes()).unwrap();
    }

    Ok(())
}
