use asmlib::instruction::*;
use std::time::Instant;
mod video;
use video::*;

pub struct Emulator {
    registers: [u16; 16],
    buffer: [u16; 32 * 32],
    next_instruction: usize,
    start_time: Instant,
    program: Vec<Instruction>,
    window: Option<Window>,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            buffer: [0; 32 * 32],
            next_instruction: 0,
            start_time: Instant::now(),
            program: Vec::new(),
            window: None,
        }
    }

    pub fn start_window(&mut self) {
        self.window = Some(Window::new())
    }

    pub fn window(&self) -> &Window {
        self.window.as_ref().unwrap()
    }

    pub fn next_instruction(&self) -> usize {
        self.next_instruction
    }

    pub fn reset_next_instruction(&mut self) {
        self.next_instruction = 0;
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program
    }

    pub fn program(&self) -> &Vec<Instruction> {
        &self.program
    }

    pub fn registers_mut(&mut self) -> &mut [u16; 16] {
        &mut self.registers
    }

    pub fn get_timer(self: &Emulator) -> u128 {
        self.start_time.elapsed().as_millis() / 30
    }

    pub fn print_all_registers(self: &mut Emulator) {
        println!("Registers :");
        for i in 0..14 {
            print!("Register {}: {}\n", i, self.registers[i]);
        }
        print!("Timer: {}\n", self.get_timer());
        println!("");
    }

    pub fn render(&mut self) {
        let window = self.window.as_mut().unwrap();

        window.update_video_buffer(&self.buffer);
    }

    pub fn execute_next_line(self: &mut Emulator) {
        if self.next_instruction >= self.program.len().try_into().unwrap() {
            panic!("[execute_next_line] try to execute next line when it is end of program");
        }

        let instruction = self.program[self.next_instruction].instruction;
        self.next_instruction += 1;

        match instruction {
            InstructionEnum::RInstruction(instruction) => match instruction.get_opcode() {
                OpCode::ADD => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        + self.registers[instruction.get_rs2() as usize]
                }
                OpCode::SUB => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        - self.registers[instruction.get_rs2() as usize]
                }
                OpCode::OR => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        | self.registers[instruction.get_rs2() as usize]
                }
                OpCode::AND => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        & self.registers[instruction.get_rs2() as usize]
                }
                OpCode::XOR => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        ^ self.registers[instruction.get_rs2() as usize]
                }
                OpCode::SLL => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        << self.registers[instruction.get_rs2() as usize]
                }
                OpCode::SRL => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        >> self.registers[instruction.get_rs2() as usize]
                }
                _ => panic!(
                    "[Emulator] unsupported OpCode \"{}\" for RInstruction",
                    instruction.get_opcode() as u32
                ),
            },
            InstructionEnum::IInstruction(instruction) => match instruction.get_opcode() {
                OpCode::ADDI => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        + instruction.get_immediate() as u16
                }
                OpCode::SUBI => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        - instruction.get_immediate() as u16
                }
                OpCode::ORI => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        | instruction.get_immediate() as u16
                }
                OpCode::ANDI => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        & instruction.get_immediate() as u16
                }
                OpCode::XORI => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        ^ instruction.get_immediate() as u16
                }
                OpCode::SLLI => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        << instruction.get_immediate() as u16
                }
                OpCode::SRLI => {
                    self.registers[instruction.get_rd() as usize] = self.registers
                        [instruction.get_rs1() as usize]
                        >> instruction.get_immediate() as u16
                }
                OpCode::LD => {
                    let rs1 = self.registers[instruction.get_rs1() as usize] as usize;
                    let rd = self.registers[instruction.get_rd() as usize] as usize;
                    let offset = instruction.get_immediate() as usize;
                    self.registers[rd] = self.buffer[rs1 + offset];
                }
                OpCode::STR => {
                    let rs1 = self.registers[instruction.get_rs1() as usize];
                    let rd = self.registers[instruction.get_rd() as usize] as usize;
                    let offset = instruction.get_immediate() as usize;
                    self.buffer[rd + offset] = rs1;
                }
                _ => panic!(
                    "[Emulator] unsupported OpCode \"{}\" for IInstruction",
                    instruction.get_opcode() as u32
                ),
            },
            InstructionEnum::BInstruction(instruction) => match instruction.get_opcode() {
                OpCode::BE => {
                    if self.registers[instruction.get_rs1() as usize]
                        == self.registers[instruction.get_rs2() as usize]
                    {
                        self.next_instruction =
                            (instruction.get_upper() << 4 | instruction.get_lower()) as usize;
                    }
                }
                OpCode::BNE => {
                    if self.registers[instruction.get_rs1() as usize]
                        != self.registers[instruction.get_rs2() as usize]
                    {
                        self.next_instruction =
                            (instruction.get_upper() << 4 | instruction.get_lower()) as usize;
                    }
                }
                _ => panic!(
                    "[Emulator] unsupported OpCode \"{}\" for IInstruction",
                    instruction.get_opcode() as u32
                ),
            },
        }
    }

    pub fn execute_all(self: &mut Emulator) {
        while self.next_instruction != self.program.len() {
            self.execute_next_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn jmp() {
        let mut instruction = BInstruction(0);
        instruction.set_opcode(OpCode::BE);
        instruction.set_lower(0xd);
        instruction.set_upper(0xabc);
        let general_instruction = decode_instruction(instruction.0);
        let mut program = Vec::new();
        program.push(general_instruction);
        let mut emulator: Emulator = Emulator::new();
        emulator.load_program(program);
        emulator.execute_all();
        assert_eq!(
            emulator.next_instruction(),
            0xabcd,
            "expected 0x{:x} got 0x{:x}",
            0xabcd,
            emulator.next_instruction()
        );
    }

    #[test]
    fn load_register() {
        let mut emulator: Emulator = Emulator::new();

        let mut instruction = IInstruction(0);
        instruction.set_opcode(OpCode::ADDI);
        instruction.set_rd(2);
        instruction.set_immediate(3);
        emulator.registers_mut()[8] = 10;
        instruction.set_rs1(8);
        emulator.program.push(Instruction {
            instruction: InstructionEnum::IInstruction(instruction),
        });
        emulator.execute_all();
        assert_eq!(emulator.registers[2], 13);
    }

    #[test]
    fn one_plus_one() {
        let mut instruction = IInstruction(0);
        instruction.set_opcode(OpCode::ADDI);
        instruction.set_immediate(10);
        instruction.set_rs1(1);
        instruction.set_rd(2);
        let mut program = Vec::new();
        program.push(Instruction {
            instruction: InstructionEnum::IInstruction(instruction),
        });
        let mut emulator: Emulator = Emulator::new();
        emulator.registers_mut()[1] = 1;
        emulator.load_program(program);
        emulator.execute_all();
        assert_eq!(emulator.registers[2], 11);
    }
}
