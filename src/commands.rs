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

pub struct WriteFifo<const N: usize>([u8; N]);

impl<const N: usize> WriteFifo<N> {
    pub const fn bytes(&self) -> [u8; N + 1] {
        let mut bytes = [0; N + 1];
        bytes[0] = 0b1000_0000;
        let mut i = 1;
        while i < N + 1 {
            bytes[i] = self.0[i - 1];
            i += 1;
        }
        bytes
    }
}

pub struct ReadFifo<const N: usize>(PhantomData<[u8; N]>);

impl<const N: usize> ReadFifo<N> {
    pub const fn bytes() -> [u8; N + 1] {
        [0; N + 1]
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

    #[test]
    fn test_write_fifo() {
        const WRITE_FIFO: WriteFifo<3> = WriteFifo([0x31, 0x92, 0x53]);
        assert_eq!(WRITE_FIFO.bytes(), [0b1000_0000, 0x31, 0x92, 0x53],);
    }

    #[test]
    fn test_read_fifo() {
        const READ_FIFO_BYTES: [u8; 4] = ReadFifo::<3>::bytes();
        assert_eq!(READ_FIFO_BYTES, [0, 0, 0, 0],);
    }
}
