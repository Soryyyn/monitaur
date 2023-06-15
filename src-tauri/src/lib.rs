#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum VCPFeatureCode {
    Brightness = 0x10,
    Saturation = 0x8a,
}
