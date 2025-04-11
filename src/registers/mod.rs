use bitfield_struct::bitfield;
pub mod lora;

#[const_trait]
pub trait Register: Copy {
    const ADDRESS: u8;

    fn into_bits(self) -> u8;
}

#[bitfield(u8, order = Msb)]
pub struct RegOpMode {
    #[bits(1)]
    pub long_range_mode: LongRangeMode,
    #[bits(1)]
    pub access_shared_reg: bool,
    #[bits(2)]
    __: u8,
    #[bits(1, default = true)]
    pub low_frequency_mode_on: bool,
    #[bits(3, default = Mode::Stdby)]
    pub mode: Mode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LongRangeMode {
    FskOok = 0,
    Lora = 1,
}
impl LongRangeMode {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-1
        unsafe { core::mem::transmute(bits & 1) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    Sleep = 0,
    Stdby = 1,
    Fstx = 2,
    Tx = 3,
    Fsrx = 4,
    RxContinuous = 5,
    RxSingle = 6,
    Cad = 7,
}
impl Mode {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-7
        unsafe { core::mem::transmute(bits & 0b111) }
    }
}

#[bitfield(u8, order = Msb)]
pub struct RegFrMsb {
    #[bits(8, default = 0x6C)]
    pub frf: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegFrMid {
    #[bits(8, default = 0x80)]
    pub frf: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegFrLsb {
    #[bits(8)]
    pub frf: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegPaConfig {
    #[bits(1)]
    pub pa_select: PaSelect,
    #[bits(3, default = 0x04)]
    pub max_power: u8,
    #[bits(4, default = 0x0f)]
    pub output_power: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PaSelect {
    Rfo = 0,
    PaBoost = 1,
}
impl PaSelect {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-1
        unsafe { core::mem::transmute(bits & 1) }
    }
}

#[bitfield(u8, order = Msb)]
pub struct RegPaRamp {
    #[bits(4)]
    __: u8,
    #[bits(4, default = PaRamp::Us40)]
    pub pa_ramp: PaRamp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PaRamp {
    Ms3_4 = 0b000,
    Ms2 = 0b0001,
    Ms1 = 0b0010,
    Us500 = 0b0011,
    Us250 = 0b0100,
    Us125 = 0b0101,
    Us100 = 0b0110,
    Us62 = 0b0111,
    Us50 = 0b1000,
    Us40 = 0b1001,
    Us31 = 0b1010,
    Us25 = 0b1011,
    Us20 = 0b1100,
    Us15 = 0b1101,
    Us12 = 0b1110,
    Us10 = 0b1111,
}
impl PaRamp {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-15
        unsafe { core::mem::transmute(bits & 0b1111) }
    }
}

#[bitfield(u8, order = Msb)]
pub struct RegOcp {
    #[bits(2)]
    __: u8,
    #[bits(1, default = true)]
    pub ocp_on: bool,
    #[bits(5, default = 0x0b)]
    pub ocp_trim: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegLna {
    #[bits(3, default = LnaGain::G1)]
    pub lna_gain: LnaGain,
    #[bits(2)]
    pub lna_boost_lf: u8,
    #[bits(1)]
    __: u8,
    #[bits(2)]
    pub lna_boost_hf: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LnaGain {
    Unused0 = 0b000,
    G1 = 0b001, // maximum gain
    G2 = 0b010,
    G3 = 0b011,
    G4 = 0b100,
    G5 = 0b101,
    G6 = 0b110, // minimum gain
    Unused1 = 0b111,
}
impl LnaGain {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-7
        unsafe { core::mem::transmute(bits & 0b111) }
    }
}

#[bitfield(u8, order = Msb)]
pub struct RegDioMapping1 {
    #[bits(2)]
    pub dio0_mapping: u8,
    #[bits(2)]
    pub dio1_mapping: u8,
    #[bits(2)]
    pub dio2_mapping: u8,
    #[bits(2)]
    pub dio3_mapping: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegDioMapping2 {
    #[bits(2)]
    pub dio4_mapping: u8,
    #[bits(2)]
    pub dio5_mapping: u8,
    #[bits(3)]
    __: u8,
    #[bits(1)]
    pub map_preamble_detect: bool,
}

impl const Register for RegOpMode {
    const ADDRESS: u8 = 0x01;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegFrMsb {
    const ADDRESS: u8 = 0x06;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegFrMid {
    const ADDRESS: u8 = 0x07;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegFrLsb {
    const ADDRESS: u8 = 0x08;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegPaConfig {
    const ADDRESS: u8 = 0x09;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegPaRamp {
    const ADDRESS: u8 = 0x0A;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegOcp {
    const ADDRESS: u8 = 0x0B;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegLna {
    const ADDRESS: u8 = 0x0C;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegDioMapping1 {
    const ADDRESS: u8 = 0x40;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegDioMapping2 {
    const ADDRESS: u8 = 0x41;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
