use ddc_hi::{traits::Ddc, Display, FeatureCode, VcpValue};
use monitaur::VCPFeatureCode;

// Values for a specific display.
#[derive(Debug)]
pub struct DisplayConf {
    pub display: Display,
    pub brightness: VcpValue,
}

// Keeps track of all displays.
pub struct Displays {
    active_displays: Vec<DisplayConf>,
}

impl Displays {
    // Constructor patter to init all displays.
    pub fn new() -> Self {
        let mut displays = Displays {
            active_displays: vec![],
        };

        // Go through each display and add it to the active displays.
        for mut display in Display::enumerate() {
            displays.active_displays.push(DisplayConf {
                brightness: displays.get_value_for_vcp(&mut display, VCPFeatureCode::Brightness),
                display,
            });
        }

        displays
    }

    pub fn get_active_displays(&self) -> &Vec<DisplayConf> {
        &self.active_displays
    }

    pub fn get_value_for_vcp(&self, display: &mut Display, code: VCPFeatureCode) -> VcpValue {
        display.handle.get_vcp_feature(code as FeatureCode).unwrap()
    }
}
