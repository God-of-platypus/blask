use core::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    Z = 0x0,
    A = 0x1,
    B = 0x2,
    C = 0x3,
    D = 0x4,
    E = 0x5,
    F = 0x6,
    G = 0x7,
    H = 0x8,
    I = 0x9,
    J = 0xA,
    K = 0xB,
    L = 0xC,
    M = 0xD,
    N = 0xE,
    S = 0xF,
}

impl TryFrom<u16> for Register {
    type Error = ();

    fn try_from(immediate: u16) -> Result<Self, ()> {
        use Register::*;

        let register = match immediate {
            0x0 => Z,
            0x1 => A,
            0x2 => B,
            0x3 => C,
            0x4 => D,
            0x5 => E,
            0x6 => F,
            0x7 => G,
            0x8 => H,
            0x9 => I,
            0xA => J,
            0xB => K,
            0xC => L,
            0xD => M,
            0xE => N,
            0xF => S,
            _ => return Err(()),
        };

        Ok(register)
    }
}

impl Index<Register> for [u16; 16] {
    type Output = u16;

    fn index(&self, register: Register) -> &Self::Output {
        &self[register as usize]
    }
}

impl IndexMut<Register> for [u16; 16] {
    fn index_mut(&mut self, register: Register) -> &mut Self::Output {
        &mut self[register as usize]
    }
}
