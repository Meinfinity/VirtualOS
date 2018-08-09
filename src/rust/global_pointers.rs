pub const REG: u32 = 4;
pub const INSTRUCTION_POINTER: u32 = 556;
pub const PREVIOUS_IP: u32 = 560;
pub const CR: u32 = 580;
pub const PREFIXES: u32 = 648;
pub const TIMESTAMP_COUNTER: u32 = 664;
pub const SEGMENT_OFFSETS: u32 = 736;
pub const OPSTATS_BUFFER: u32 = 0x1000;
pub const OPSTATS_BUFFER_0F: u32 = 0x1400;
pub const TLB_DATA: u32 = 0x400000;

pub fn get_reg16_offset(r: u32) -> u32 {
    dbg_assert!(r < 8);
    REG + 4 * r
}
pub fn get_reg32_offset(r: u32) -> u32 {
    dbg_assert!(r < 8);
    REG + 4 * r
}

pub fn get_seg_offset(s: u32) -> u32 {
    dbg_assert!(s < 8);
    SEGMENT_OFFSETS + 4 * s
}

pub fn get_creg_offset(cr: u32) -> u32 {
    dbg_assert!(cr < 8);
    CR + 4 * cr
}
