use anyhow::{Context, Result};
use ddc_hi::{traits::Ddc, Display, FeatureCode, VcpValue};
use monitaur::VCPFeatureCode;

#[derive(Debug)]
pub struct DisplayConf {
    pub display: Display,
    pub brightness: VcpValue,
}

pub struct Displays {
    active_displays: Vec<DisplayConf>,
}

impl Displays {
    pub fn new() -> Result<Self> {
        let mut displays = Displays {
            active_displays: vec![],
        };

        for mut display in Display::enumerate() {
            let brightness =
                displays.get_value_for_vcp(&mut display, VCPFeatureCode::Brightness)?;

            displays.active_displays.push(DisplayConf {
                brightness,
                display,
            });
        }

        Ok(displays)
    }

    pub fn get_active_displays(&self) -> &Vec<DisplayConf> {
        &self.active_displays
    }

    // Will return and error if something failed while tring to get the value
    // for the wanted code.
    pub fn get_value_for_vcp(
        &self,
        display: &mut Display,
        code: VCPFeatureCode,
    ) -> Result<VcpValue> {
        display
            .handle
            .get_vcp_feature(code as FeatureCode)
            .with_context(|| format!("Failed getting VCP value for code {:?}", code))
    }
}
