use crate::*;

use blex::Lexer;

#[test]
fn test_instr_add() {
    let text = "add 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::ADD(Register::R0, Register::R1, Register::R2)
    );
}

#[test]
fn test_instr_sub() {
    let text = "sub 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::SUB(Register::R0, Register::R1, Register::R2)
    );
}

#[test]
fn test_instr_or() {
    let text = "or 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::OR(Register::R0, Register::R1, Register::R2)
    );
}

#[test]
fn test_instr_and() {
    let text = "and 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::AND(Register::R0, Register::R1, Register::R2)
    );
}

#[test]
fn test_instr_xor() {
    let text = "xor 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::XOR(Register::R0, Register::R1, Register::R2)
    );
}

#[test]
fn test_instr_sll() {
    let text = "sll 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::SLL(Register::R0, Register::R1, Register::R2)
    );
}

#[test]
fn test_instr_srl() {
    let text = "srl 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::SRL(Register::R0, Register::R1, Register::R2)
    );
}

#[test]
fn test_instr_addi() {
    let text = "addi 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    assert_eq!(
        asm.next(),
        Some(Ok(PseudoInstruction::ADDI(Register::R0, Register::R1, 2)))
    );
    assert_eq!(asm.next(), None);
}

#[test]
fn test_instr_subi() {
    let text = "subi 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::SUBI(Register::R0, Register::R1, 2)
    );
}

#[test]
fn test_instr_ori() {
    let text = "ori 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::ORI(Register::R0, Register::R1, 2)
    );
}

#[test]
fn test_instr_andi() {
    let text = "andi 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::ANDI(Register::R0, Register::R1, 2)
    );
}

#[test]
fn test_instr_xori() {
    let text = "xori 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::XORI(Register::R0, Register::R1, 2)
    );
}

#[test]
fn test_instr_slli() {
    let text = "slli 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::SLLI(Register::R0, Register::R1, 2)
    );
}

#[test]
fn test_instr_srli() {
    let text = "srli 0, 1, 2\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::SRLI(Register::R0, Register::R1, 2)
    );
}

#[test]
fn test_instr_be() {
    let text = "be 0, 1, -16\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::BE(Register::R0, Register::R1, -16)
    );
}

#[test]
fn test_instr_bne() {
    let text = "bne 0, 1, 16\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::BNE(Register::R0, Register::R1, 16)
    );
}

#[test]
fn test_instr_blt() {
    let text = "blt 0, 1, -10\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::BLT(Register::R0, Register::R1, -10)
    );
}

#[test]
fn test_instr_bge() {
    let text = "bge 0, 1, 10\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::BGE(Register::R0, Register::R1, 10)
    );
}

#[test]
fn test_instr_bltu() {
    let text = "bltu 0, 1, 10\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::BLTU(Register::R0, Register::R1, 10)
    );
}

#[test]
fn test_instr_bgeu() {
    let text = "bgeu 0, 1, 1\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::BGEU(Register::R0, Register::R1, 1)
    );
}

#[test]
fn test_label() {
    let text = "@label\naddi 0, 1, 2\nbgeu 0, 1, @label\n";
    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::ADDI(Register::R0, Register::R1, 2)
    );

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::BGEU(Register::R0, Register::R1, 0)
    );
}

#[test]
fn test_label_comment() {
    let text = r##"#This is a comment
@label
addi 0, 1, 2 # Another comment
bgeu 0, 1, @label # Another comment
# Another Comment"##;

    let lexer = Lexer::new(&text);
    let mut asm = ASM::new(&text, &lexer);

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::ADDI(Register::R0, Register::R1, 2)
    );

    let pseudo_instr = asm.next().unwrap().unwrap();

    assert_eq!(
        pseudo_instr,
        PseudoInstruction::BGEU(Register::R0, Register::R1, 0)
    );
}
