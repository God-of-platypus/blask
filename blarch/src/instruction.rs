use blib::Register;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    // R-Type Instructions
    ADD(Register, Register, Register),
    SUB(Register, Register, Register),
    OR(Register, Register, Register),
    AND(Register, Register, Register),
    XOR(Register, Register, Register),
    // I-Type Instructions
    ADDI(Register, Register, u16),
    SUBI(Register, Register, u16),
    ORI(Register, Register, u16),
    ANDI(Register, Register, u16),
    XORI(Register, Register, u16),
    // B-Type Instructions
    JEQ(Register, Register, i16),
    JNE(Register, Register, i16),
    JLT(Register, Register, i16),
    JGE(Register, Register, i16),
    JLTS(Register, Register, i16),
    JGES(Register, Register, i16),
    JAL(Register, Register, i16),
}
