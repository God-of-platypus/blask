#[derive(Debug)]
pub enum OpcodeI {
    Lui = 0x37,
    Auipc = 0x17,
    ImmOp = 0x13,
    Branch = 0x63,
    Load = 0x03,
    Store = 0x23,
    Jal = 0x6f,
    Jalr = 0x67,
    RegOp = 0x33,
}

#[derive(Debug)]
pub enum ImmOpFunc3I {
    Addi = 0x0,
    Slli = 0x1,
    Slti = 0x2,
    Stliu = 0x3,
    Xori = 0x4,
    Func7 = 0x5,
    Ori = 0x6,
    Andi = 0x7,
}

#[derive(Debug)]
pub enum ImmOpFunc7I {
    Slri = 0x00,
    Srai = 0x20,
}

#[derive(Debug)]
pub enum BranchFunc3I {
    Beq = 0x0,
    Bne = 0x1,
    Blt = 0x4,
    Bge = 0x5,
    Bltu = 0x6,
    Bgeu = 0x7,
}

#[derive(Debug)]
pub enum LoadFunc3I {
    Lb = 0x0,
    Lh = 0x1,
    Lw = 0x2,
    Lbu = 0x4,
    Lhu = 0x5
}

#[derive(Debug)]
pub enum StoreFunc3I {
    Sb = 0x0,
    Sh = 0x1,
    Sw = 0x2,
}

#[derive(Debug)]
pub enum RegOppFunc3I {
    AddOrSub = 0x0,
    Sll = 0x1,
    Slt = 0x2,
    Sltu = 0x3,
    Xor = 0x4,
    Sr = 0x5,
    Or = 0x6,
    And = 0x7,
}

#[derive(Debug)]
pub enum AddOrSubFunc7I {
    Add = 0x0,
    Sub = 0x20,
}

#[derive(Debug)]
pub enum SrFunc7I {
    Srl = 0x0,
    Sra = 0x20,
}

