// VCP code to access via ddc.
#[repr(u8)]
pub enum VCPFeatureCode {
    Brightness = 0x10,
    Saturation = 0x8a,
}
