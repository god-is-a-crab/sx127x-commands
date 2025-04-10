use crate::registers::Register;
use core::marker::PhantomData;

pub struct WriteSingle<T: Register>(T);

impl<T: const Register> WriteSingle<T> {
    pub const fn bytes(&self) -> [u8; 2] {
        [T::ADDRESS | 0b1000_0000, self.0.into_bits()]
    }
}

pub struct ReadSingle<T: Register>(PhantomData<T>);

impl<T: const Register> ReadSingle<T> {
    pub const fn bytes() -> [u8; 2] {
        [T::ADDRESS, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers;

    #[test]
    fn test_write_single() {
        const OP_MODE: registers::RegOpMode = registers::RegOpMode::new()
            .with_long_range_mode(registers::LongRangeMode::Lora)
            .with_access_shared_reg(false)
            .with_low_frequency_mode_on(true)
            .with_mode(registers::Mode::Tx);
        assert_eq!(OP_MODE.into_bits(), 0b1000_1011,);

        const WRITE_SINGLE: WriteSingle<registers::RegOpMode> = WriteSingle(OP_MODE);
        assert_eq!(WRITE_SINGLE.bytes(), [0x01 | 0b1000_0000, 0b1000_1011],);
    }
}
