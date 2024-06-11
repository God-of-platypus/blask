
pub struct Cpu {
    pub registers: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u32>,
    pub csrs: [u32; 4096],
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            memory: vec![0; 1024 * 1024], // 1MB memory for example
            csrs: [0; 4096],
        }
    }

}

impl Cpu {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers : [0;32],
            pc: 0x0,
            memory: vec![0; memory_size],
            csrs: [0;4096],
        }
    }

    fn fetch(&self) -> u32 {
        let pc = self.pc as usize;
        (self.memory[pc] as u32)
            | ((self.memory[pc + 1] as u32) << 8)
            | ((self.memory[pc + 2] as u32) << 16)
            | ((self.memory[pc + 3] as u32) << 24)
    }
}