use std::time::Duration;
use std::time::Instant;

// https://stackoverflow.com/questions/20922091/how-do-you-use-parent-module-imports-in-rust
use asmlib::instruction::Instruction;
use asmlib::instruction::InstructionEnum;
use clap::Parser;
use emulator::Emulator;
use files;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file that the Emulator will execute.
    #[arg(short, long)]
    path: String,

    /// Set Emulator to Debug mode.
    #[arg(short, long)]
    debug: bool,

    /// Dump all the registers at the end of the execution
    #[arg(long)]
    dump_regs: bool,

    /// Enable the Window for the Emulator
    ///
    /// This will execute your input file every frame.
    #[arg(short, long)]
    video: bool,
}

fn print_instruction(emulator: &Emulator) {
    print!(
        "Executing opcode : {} with ",
        emulator.program()[emulator.next_instruction()].get_opcode()
    );
    match emulator.program()[emulator.next_instruction()].instruction {
        InstructionEnum::RInstruction(instruction) => println!(
            "rd {} rs1 {} rs2 {} ",
            instruction.get_rd(),
            instruction.get_rs1(),
            instruction.get_rs2()
        ),
        InstructionEnum::IInstruction(instruction) => println!(
            "rd {} rs1 {} immediate {} ",
            instruction.get_rd(),
            instruction.get_rs1(),
            instruction.get_immediate()
        ),
        InstructionEnum::BInstruction(instruction) => println!(
            "rs1 {} rs2 {} ",
            instruction.get_rs1(),
            instruction.get_rs2()
        ),
    }
}

fn main() {
    let args = Args::parse();

    let program: Vec<Instruction>;

    if args.path.ends_with(".blasm") {
        program = files::blasm_to_instructions(args.path.as_str());
    } else if args.path.ends_with(".bin") {
        program = files::binary_to_instructions(args.path.as_str());
    } else {
        panic!("File must end with blasm or bin");
    }

    if args.debug {
        println!("Program size : {}", program.len());
    }

    let mut emulator: Emulator = Emulator::new();
    emulator.load_program(program);

    if args.debug {
        println!("Executing file {}...", args.path);
        println!("Type p to print registers n to go to next instruction");

        while emulator.next_instruction() < emulator.program().len().try_into().unwrap() {
            let mut command = String::new();
            std::io::stdin().read_line(&mut command).unwrap();
            println!("{}", emulator.get_timer());

            match command.as_str().trim() {
                "p" => emulator.print_all_registers(),
                "n" => {
                    print_instruction(&emulator);
                    emulator.execute_next_line();
                }
                _ => println!("Not recognized"),
            }
        }
        emulator.print_all_registers();
        println!("End of program");
    } else if args.video {
        let frame_time = Duration::from_millis(1_000 / 30);
        let mut frame = 0;

        emulator.start_window();

        while !emulator.window().window_should_close() {
            format!("Frame: {}\r", frame);

            // 0. Get time at start of frame
            let start = Instant::now();

            // 1. Fetch the inputs
            // TODO

            // 2. Execute the program
            emulator.reset_next_instruction(); // Set program counter to the start of the program
            emulator.execute_all();

            // 3. Update the Video buffer
            emulator.render();

            // 4. Wait for the frame time
            let end = Instant::now();

            let time_elapsed = end - start;

            if time_elapsed < frame_time {
                std::thread::sleep(frame_time - time_elapsed);
            }

            frame += 1;
        }
    } else {
        emulator.execute_all();

        if args.dump_regs {
            emulator.print_all_registers();
        }
    }
}
