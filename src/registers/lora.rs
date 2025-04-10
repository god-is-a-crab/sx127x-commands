use super::Register;
use bitfield_struct::bitfield;

#[bitfield(u8, order = Msb)]
pub struct RegFifoAddrPtr {
    #[bits(8)]
    pub fifo_addr_ptr: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegFifoTxBaseAddr {
    #[bits(8, default = 0x80)]
    pub fifo_tx_base_addr: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegFifoRxBaseAddr {
    #[bits(8)]
    pub fifo_rx_base_addr: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegFifoRxCurrentAddr {
    #[bits(8, access = RO)]
    pub fifo_rx_current_addr: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegIrqFlagsMask {
    #[bits(1)]
    pub rx_timeout_mask: bool,
    #[bits(1)]
    pub rx_done_mask: bool,
    #[bits(1)]
    pub payload_crc_error_mask: bool,
    #[bits(1)]
    pub valid_header_mask: bool,
    #[bits(1)]
    pub tx_done_mask: bool,
    #[bits(1)]
    pub cad_done_mask: bool,
    #[bits(1)]
    pub fhss_change_channel_mask: bool,
    #[bits(1)]
    pub cad_detected_mask: bool,
}

#[bitfield(u8, order = Msb)]
pub struct RegIrqFlags {
    #[bits(1)]
    pub rx_timeout: bool,
    #[bits(1)]
    pub rx_done: bool,
    #[bits(1)]
    pub payload_crc_error: bool,
    #[bits(1)]
    pub valid_header: bool,
    #[bits(1)]
    pub tx_done: bool,
    #[bits(1)]
    pub cad_done: bool,
    #[bits(1)]
    pub fhss_change_channel: bool,
    #[bits(1)]
    pub cad_detected: bool,
}

#[bitfield(u8, order = Msb)]
pub struct RegRxNbBytes {
    #[bits(8, access = RO)]
    pub fifo_rx_bytes_nb: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegRxHeaderCntValueMsb {
    #[bits(8, access = RO)]
    pub valid_header_cnt_msb: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegRxHeaderCntValueLsb {
    #[bits(8, access = RO)]
    pub valid_header_cnt_lsb: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegRxPacketCntValueMsb {
    #[bits(8, access = RO)]
    pub valid_packet_cnt_msb: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegRxPacketCntValueLsb {
    #[bits(8, access = RO)]
    pub valid_packet_cnt_lsb: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegModemStat {
    #[bits(3, access = RO)]
    pub rx_coding_rate: u8,
    #[bits(1, access = RO)]
    pub modem_clear: bool,
    #[bits(1, access = RO)]
    pub header_info_valid: bool,
    #[bits(1, access = RO)]
    pub rx_ongoing: bool,
    #[bits(1, access = RO)]
    pub signal_synchronized: bool,
    #[bits(1, access = RO)]
    pub signal_detected: bool,
}

#[bitfield(u8, order = Msb)]
pub struct RegPktSnrValue {
    #[bits(8, access = RO)]
    pub packet_snr: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegPktRssiValue {
    #[bits(8, access = RO)]
    pub packet_rssi: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegRssiValue {
    #[bits(8, access = RO)]
    pub rssi: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegHopChannel {
    #[bits(1, access = RO)]
    pub pll_timeout: bool,
    #[bits(1, access = RO)]
    pub crc_on_payload: bool,
    #[bits(6, access = RO)]
    pub fhss_present_channel: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegModemConfig1 {
    #[bits(4, default = Bw::Bw125KHz)]
    pub bw: Bw,
    #[bits(3, default = CodingRate::Cr4_5)]
    pub coding_rate: CodingRate,
    #[bits(1)]
    pub implicit_header_mode: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Bw {
    Bw7_8KHz = 0b0000,
    Bw10_4KHz = 0b0001,
    Bw15_6KHz = 0b0010,
    Bw20_8KHz = 0b0011,
    Bw31_25KHz = 0b0100,
    Bw41_7KHz = 0b0101,
    Bw62_5KHz = 0b0110,
    Bw125KHz = 0b0111,
    Bw250KHz = 0b1000,
    Bw500KHz = 0b1001,
    Reserved0 = 0b1010,
    Reserved1 = 0b1011,
    Reserved2 = 0b1100,
    Reserved3 = 0b1101,
    Reserved4 = 0b1110,
    Reserved5 = 0b1111,
}
impl Bw {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-15
        unsafe { std::mem::transmute(bits & 0b1111) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CodingRate {
    Reserved0 = 0b000,
    Cr4_5 = 0b001,
    Cr4_6 = 0b010,
    Cr4_7 = 0b011,
    Cr4_8 = 0b100,
    Reserved1 = 0b101,
    Reserved2 = 0b110,
    Reserved3 = 0b111,
}
impl CodingRate {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-7
        unsafe { std::mem::transmute(bits & 0b111) }
    }
}

#[bitfield(u8, order = Msb)]
pub struct RegModemConfig2 {
    #[bits(4, default = 0x07)]
    pub spreading_factor: u8,
    #[bits(1)]
    pub tx_continuous_mode: bool,
    #[bits(1)]
    pub rx_payload_crc_on: bool,
    #[bits(2)]
    pub symb_timeout_msb: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegSymbTimeoutLsb {
    #[bits(8, default = 0x64)]
    pub symb_timeout: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegPreambleMsb {
    #[bits(8)]
    pub preamble_length: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegPreambleLsb {
    #[bits(8, default = 0x08)]
    pub preamble_length: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegPayloadLength {
    #[bits(8, default = 1)]
    pub payload_length: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegMaxPayloadLength {
    #[bits(8, default = 0xff)]
    pub payload_max_length: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegHopPeriod {
    #[bits(8)]
    pub freq_hopping_period: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegFifoRxByteAddr {
    #[bits(8, access = RO)]
    pub fifo_rx_byte_addr_ptr: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegModemConfig3 {
    #[bits(4)]
    __: u8,
    #[bits(1)]
    pub low_data_rate_optimize: bool,
    #[bits(1)]
    pub agc_auto_on: bool,
    #[bits(2)]
    __: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegPpmCorrection {
    #[bits(8)]
    pub ppm_correction: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegFeiMsb {
    #[bits(4)]
    __: u8,
    #[bits(4, access = RO)]
    pub freq_error: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegFeiMid {
    #[bits(8, access = RO)]
    pub freq_error: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegFeiLsb {
    #[bits(8, access = RO)]
    pub freq_error: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegRssiWideband {
    #[bits(8, access = RO)]
    pub rssi_wideband: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegIfFreq2 {
    #[bits(8, default = 0x20)]
    pub if_freq2: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegIfFreq1 {
    #[bits(8)]
    pub if_freq1: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegDetectOptimize {
    #[bits(1, default = false)]
    pub automatic_if_on: bool,
    #[bits(4)]
    __: u8,
    #[bits(3, default = DetectionOptimize::Sf7ToSf12)]
    pub detection_optimize: DetectionOptimize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DetectionOptimize {
    Reserved0 = 0x00,
    Reserved1 = 0x01,
    Reserved2 = 0x02,
    Sf7ToSf12 = 0x03,
    Reserved3 = 0x04,
    Sf6 = 0x05,
    Reserved4 = 0x06,
    Reserved5 = 0x07,
}
impl DetectionOptimize {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-7
        unsafe { std::mem::transmute(bits & 0b111) }
    }
}

#[bitfield(u8, order = Msb)]
pub struct RegInvertIq {
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub invert_iq_rx: bool,
    #[bits(5)]
    __: u8,
    #[bits(1)]
    pub invert_iq_tx: bool,
}

#[bitfield(u8, order = Msb)]
pub struct RegHighBwOptimize1 {
    #[bits(8, default = 0x20)]
    pub high_bw_optimize1: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegDetectionThreshold {
    #[bits(8, default = DetectionThreshold::Sf7ToSf12)]
    pub detection_threshold: DetectionThreshold,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DetectionThreshold {
    Reserved0 = 0x00,
    Reserved1 = 0x01,
    Reserved2 = 0x02,
    Reserved3 = 0x03,
    Reserved4 = 0x04,
    Reserved5 = 0x05,
    Reserved6 = 0x06,
    Reserved7 = 0x07,
    Reserved8 = 0x08,
    Reserved9 = 0x09,
    Sf7ToSf12 = 0x0A,
    Reserved10 = 0x0B,
    Sf6 = 0x0C,
    Reserved11 = 0x0D,
    Reserved12 = 0x0E,
    Reserved13 = 0x0F,
}
impl DetectionThreshold {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-15
        unsafe { std::mem::transmute(bits & 0b1111) }
    }
}

#[bitfield(u8, order = Msb)]
pub struct RegSyncWord {
    #[bits(8, default = 0x12)]
    pub sync_word: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegHighBwOptimize2 {
    #[bits(8, default = 0x20)]
    pub high_bw_optimize2: u8,
}

#[bitfield(u8, order = Msb)]
pub struct RegInvertIq2 {
    #[bits(8, default = 0x1D)]
    pub invert_iq2: u8,
}

impl const Register for RegFifoAddrPtr {
    const ADDRESS: u8 = 0x0D;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegFifoTxBaseAddr {
    const ADDRESS: u8 = 0x0E;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegFifoRxBaseAddr {
    const ADDRESS: u8 = 0x0F;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegFifoRxCurrentAddr {
    const ADDRESS: u8 = 0x10;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegIrqFlagsMask {
    const ADDRESS: u8 = 0x11;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegIrqFlags {
    const ADDRESS: u8 = 0x12;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegRxNbBytes {
    const ADDRESS: u8 = 0x13;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegRxHeaderCntValueMsb {
    const ADDRESS: u8 = 0x14;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegRxHeaderCntValueLsb {
    const ADDRESS: u8 = 0x15;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegRxPacketCntValueMsb {
    const ADDRESS: u8 = 0x16;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegRxPacketCntValueLsb {
    const ADDRESS: u8 = 0x17;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegModemStat {
    const ADDRESS: u8 = 0x18;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegPktSnrValue {
    const ADDRESS: u8 = 0x19;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegPktRssiValue {
    const ADDRESS: u8 = 0x1A;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegRssiValue {
    const ADDRESS: u8 = 0x1B;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegHopChannel {
    const ADDRESS: u8 = 0x1C;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegModemConfig1 {
    const ADDRESS: u8 = 0x1D;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegModemConfig2 {
    const ADDRESS: u8 = 0x1E;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegSymbTimeoutLsb {
    const ADDRESS: u8 = 0x1F;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegPreambleMsb {
    const ADDRESS: u8 = 0x20;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegPreambleLsb {
    const ADDRESS: u8 = 0x21;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegPayloadLength {
    const ADDRESS: u8 = 0x22;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegMaxPayloadLength {
    const ADDRESS: u8 = 0x23;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegHopPeriod {
    const ADDRESS: u8 = 0x24;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegFifoRxByteAddr {
    const ADDRESS: u8 = 0x25;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegModemConfig3 {
    const ADDRESS: u8 = 0x26;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegFeiMsb {
    const ADDRESS: u8 = 0x28;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegFeiMid {
    const ADDRESS: u8 = 0x29;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegFeiLsb {
    const ADDRESS: u8 = 0x2A;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegRssiWideband {
    const ADDRESS: u8 = 0x2C;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegIfFreq2 {
    const ADDRESS: u8 = 0x2F;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegIfFreq1 {
    const ADDRESS: u8 = 0x30;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegDetectOptimize {
    const ADDRESS: u8 = 0x31;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegInvertIq {
    const ADDRESS: u8 = 0x33;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegHighBwOptimize1 {
    const ADDRESS: u8 = 0x36;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegDetectionThreshold {
    const ADDRESS: u8 = 0x37;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegSyncWord {
    const ADDRESS: u8 = 0x39;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegHighBwOptimize2 {
    const ADDRESS: u8 = 0x3A;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
impl const Register for RegInvertIq2 {
    const ADDRESS: u8 = 0x3B;

    fn into_bits(self) -> u8 {
        self.into_bits()
    }
}
