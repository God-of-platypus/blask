use cpu::Cpu;

pub fn lui(_cpu: &mut Cpu, instruction : u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let imm_u = instruction & 0xfffff000;
    _cpu.registers[rd] = imm_u;
}

pub fn auipc(_cpu: &mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let imm_u = instruction & 0xfffff000;
    _cpu.registers[rd] = _cpu.pc.wrapping_add(imm_u);
}

pub fn addi(_cpu: &mut Cpu, instruction:u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let imm_i = ((instruction as i32) >> 20) as u32;
    _cpu.registers[rd] = _cpu.registers[rs1].wrapping_add(imm_i);
}

pub fn stli(_cpu : &mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let imm_i = ((instruction as i32) >> 20) as u32;
    _cpu.registers[rd] = if (_cpu.registers[rs1] as i32) < (imm_i as i32) {1} else {0};
}

pub fn stliu(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let imm_i = ((instruction as i32) >> 20) as u32;
    _cpu.registers[rd] = if _cpu.registers[rs1] < imm_i {1} else {0};
}

pub fn xori(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let imm_i = ((instruction as i32) >> 20) as u32;
    _cpu.registers[rd] = _cpu.registers[rs1] ^ imm_i;
}

pub fn ori(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let imm_i = ((instruction as i32) >> 20) as u32;
    _cpu.registers[rd] = _cpu.registers[rs1] | imm_i;
}

pub fn andi(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let imm_i = ((instruction as i32) >> 20) as u32;
    _cpu.registers[rd] = _cpu.registers[rs1] & imm_i;
}

pub fn slli (_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let shamt = (instruction >> 20) & 0xf;
    _cpu.registers[rd] = _cpu.registers[rs1] << shamt;
}

pub fn srli (_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let shamt = (instruction >> 20) & 0xf;
    _cpu.registers[rd] = _cpu.registers[rs1] >> shamt;
}


pub fn srai(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let shamt = (instruction >> 20) & 0xf;
    let tmp = (_cpu.registers[rs1] as i32) >> shamt;
    _cpu.registers[rd] = tmp as u32;
}

pub fn add(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let rs2 = ((instruction >> 20) & 0x1f) as usize;
    _cpu.registers[rd] = _cpu.registers[rs1].wrapping_add(_cpu.registers[rs2]);
}

pub fn sub(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let rs2 = ((instruction >> 20) & 0x1f) as usize;
    _cpu.registers[rd] = _cpu.registers[rs1].wrapping_sub(_cpu.registers[rs2]);
}

pub fn sll(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let rs2 = ((instruction >> 20) & 0x1f) as usize;
    _cpu.registers[rd] = _cpu.registers[rs1] << (_cpu.registers[rs2] & 0x1f);
}

pub fn slt(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let rs2 = ((instruction >> 20) & 0x1f) as usize;
    _cpu.registers[rd] = if (_cpu.registers[rs1] as i32) < (_cpu.registers[rs2] as i32) {1} else {0};
}

pub fn sltu(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let rs2 = ((instruction >> 20) & 0x1f) as usize;
    _cpu.registers[rd] = if _cpu.registers[rs1]< _cpu.registers[rs2] {1} else {0};
}

pub fn xor(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let rs2 = ((instruction >> 20) & 0x1f) as usize;
    _cpu.registers[rd] = _cpu.registers[rs1] ^ _cpu.registers[rs2];
}

pub fn srl(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let rs2 = ((instruction >> 20) & 0x1f) as usize;
    _cpu.registers[rd] = _cpu.registers[rs1] >> (_cpu.registers[rs2] & 0x1f);
}

pub fn sra(_cpu: &mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let rs2 = ((instruction >> 20) & 0x1f) as usize;
    _cpu.registers[rd] = ((_cpu.registers[rs1] as i32) >> (_cpu.registers[rs2] & 0x1f)) as u32;
}

pub fn or(_cpu:&mut Cpu, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1f) as usize;
    let rs1 = ((instruction >> 15) & 0x1f) as usize;
    let rs2 = ((instruction >> 20) & 0x1f) as usize;
    _cpu.registers[rd] = _cpu.registers[rs1] | _cpu.registers[rs2];
}

#[cfg(test)]
mod tests {
    use cpu::Cpu;
    use crate::riscvi::*;

    #[test]
    fn lui_test() {
        let mut cpu = Cpu::new(1024);
        let instruction: u32 = 0x000FF037;
        lui(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0x000FF000);
    }

    #[test]
    fn auipc_test() {
        let mut cpu = Cpu::new(1024);
        let instruction:u32 = 0x000FF017;
        cpu.pc = 0x17;
        auipc(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0x000FF017);
    }
    #[test]
    fn addi_test() {
        let mut cpu = Cpu::new(1024);
        cpu.registers[1] =  0x12;
        let instruction = 0x12308013;
        addi(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0x135);
    }

    #[test]
    fn stli_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x1230A013;
        cpu.registers[1] = 1;
        stli(&mut cpu, instruction);
        assert_ne!(cpu.registers[0], 0);
        cpu.registers[1] = 0xfff;
        stli(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0);
    }

    #[test]
    fn stliu_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x1230A013;
        cpu.registers[1] = 1;
        stliu(&mut cpu, instruction);
        assert_ne!(cpu.registers[0], 0);
        cpu.registers[1] = 0xfff;
        stliu(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0);
    }

    #[test]
    fn xori_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x1230C013;
        cpu.registers[1] = 0x27;
        xori(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0x104);
    }

    #[test]
    fn ori_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x1230E013;
        cpu.registers[1] = 0x27;
        ori(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0x127);
    }

    #[test]
    fn andi_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x1230F013;
        cpu.registers[1] = 0x27;
        andi(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0x23);
    }

    #[test]
    fn slli_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x00409013;
        cpu.registers[1] = 0x27;
        slli(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0x270);
    }

    #[test]
    fn srli_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x0040D013;
        cpu.registers[1] = 0x27;
        srli(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0x2);
    }

    #[test]
    fn srai_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x00F0D013;
        cpu.registers[1] = 0xFFFFFFFE;
        srai(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0xFFFFFFFF);
    }

    #[test]
    fn add_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x00008033;
        cpu.registers[1] = 12;
        add(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 12);
    }

    #[test]
    fn sub_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x40008033;
        cpu.registers[0] = 6;
        cpu.registers[1] = 12;
        sub(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 6);
    }

    #[test]
    fn sll_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x00209033;
        cpu.registers[1] = 1;
        cpu.registers[2] = 4;
        sll(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0x10);
    }

    #[test]
    fn slt_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x0020A033;
        cpu.registers[1] = 12;
        cpu.registers[2] = 2;
        slt(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 0);
        cpu.registers[1] = 0xFFFFFFFF;
        slt(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 1);
    }

    #[test]
    fn sltu_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x0020B033;
        cpu.registers[1] = 1;
        cpu.registers[2] = 2;
        sltu(&mut cpu, instruction);
        assert_eq!(cpu.registers[0], 1);
        cpu.registers[1] = 0xFFFFFFFF;
        sltu(&mut cpu, instruction);
        assert_eq!(cpu.registers[0],0);
    }

    #[test]
    fn xor_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x0020C033;
        cpu.registers[1] = 1;
        cpu.registers[2] = 4;
        xor(&mut cpu, instruction);
        assert_eq!(cpu.registers[0],5);
    }

    #[test]
    fn srl_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x0020D033;
        cpu.registers[1] = 0xFFF;
        cpu.registers[2] = 4;
        srl(&mut cpu, instruction);
        assert_eq!(cpu.registers[0],0xFF);
    }

    #[test]
    fn sra_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x4020D033;
        cpu.registers[1] = 0xFFFFFFFE;
        cpu.registers[2] = 1;
        sra(&mut cpu, instruction);
        assert_eq!(cpu.registers[0],0xFFFFFFFF);
    }

    #[test]
    fn or_test() {
        let mut cpu = Cpu::new(1024);
        let instruction = 0x0020E033;
        cpu.registers[1] = 0xAAAAAAAA;
        cpu.registers[2] = 0x55555555;
        or(&mut cpu, instruction);
        assert_eq!(cpu.registers[0],0xFFFFFFFF);
    }
}