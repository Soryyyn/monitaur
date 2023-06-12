use ddc_hi::{traits::Ddc, Display, FeatureCode, VcpValue};

#[derive(Debug)]
pub struct DisplayConf {
    pub display: Display,
    pub brightness: VcpValue,
}

pub struct Displays {
    active_displays: Vec<DisplayConf>,
}

impl Displays {
    // Initialize the displays.
    pub fn init_displays() -> Displays {
        let mut displays = Displays {
            active_displays: vec![],
        };

        // Go through each display and add it to the active displays.
        for mut display in Display::enumerate() {
            displays.active_displays.push(DisplayConf {
                brightness: displays.get_value_for_vcp(&mut display, 0x10),
                display,
            });
        }

        displays
    }

    // Get all active displays.
    pub fn get_active_displays(&self) -> &Vec<DisplayConf> {
        &self.active_displays
    }

    // Get the value for a vcp feature.
    pub fn get_value_for_vcp(&self, display: &mut Display, vcp_code: FeatureCode) -> VcpValue {
        display.handle.get_vcp_feature(vcp_code).unwrap()
    }
}
