// Deployment GPIO pins + Values are Linux global
// GPIO numbers. Cheatsheet "GPIO 1..12" labels are not these numbers.

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Output,
    Input,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Active {
    High,
    Low,
}

// Antenna deployment
pub const VHF_DEPLOY_OUT: u32 = 117;
pub const VHF_DEPLOYED_SENSE: u32 = 66;
pub const UHF_DEPLOY_OUT: u32 = 115;
pub const UHF_DEPLOYED_SENSE: u32 = 7;

// Solar panel sense (active-low)
pub const SOLAR_PX_SENSE: u32 = 68;
pub const SOLAR_NX_SENSE: u32 = 69;

// GNSS
pub const GNSS_ENABLE: u32 = 46; // active-low
pub const GNSS_RESET: u32 = 60;

// SNN
pub const SNN_ENABLE: u32 = 61;
pub const SNN_RESET: u32 = 48; // active-low

// CNN
pub const CNN_BOOT: u32 = 67;

// ADCS
pub const ADCS_ENABLE: u32 = 47; // active-low

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PinInfo {
    pub name: &'static str,
    pub gpio: u32,
    pub direction: Direction,
    pub active: Active,
}

pub const PIN_TABLE: [PinInfo; 12] = [
    PinInfo { name: "VHF_DEPLOY_OUT",     gpio: VHF_DEPLOY_OUT,     direction: Direction::Output, active: Active::High },
    PinInfo { name: "VHF_DEPLOYED_SENSE", gpio: VHF_DEPLOYED_SENSE, direction: Direction::Input,  active: Active::High },
    PinInfo { name: "UHF_DEPLOY_OUT",     gpio: UHF_DEPLOY_OUT,     direction: Direction::Output, active: Active::High },
    PinInfo { name: "UHF_DEPLOYED_SENSE", gpio: UHF_DEPLOYED_SENSE, direction: Direction::Input,  active: Active::High },
    PinInfo { name: "SOLAR_PX_SENSE",     gpio: SOLAR_PX_SENSE,     direction: Direction::Input,  active: Active::Low  },
    PinInfo { name: "SOLAR_NX_SENSE",     gpio: SOLAR_NX_SENSE,     direction: Direction::Input,  active: Active::Low  },
    PinInfo { name: "GNSS_ENABLE",        gpio: GNSS_ENABLE,        direction: Direction::Output, active: Active::Low  },
    PinInfo { name: "GNSS_RESET",         gpio: GNSS_RESET,         direction: Direction::Output, active: Active::Low  },
    PinInfo { name: "SNN_ENABLE",         gpio: SNN_ENABLE,         direction: Direction::Output, active: Active::High },
    PinInfo { name: "SNN_RESET",          gpio: SNN_RESET,          direction: Direction::Output, active: Active::Low  },
    PinInfo { name: "CNN_BOOT",           gpio: CNN_BOOT,           direction: Direction::Output, active: Active::High },
    PinInfo { name: "ADCS_ENABLE",        gpio: ADCS_ENABLE,        direction: Direction::Output, active: Active::Low  },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn antenna_numbers_match_pin_mapping() {
        assert_eq!(VHF_DEPLOY_OUT, 117);
        assert_eq!(VHF_DEPLOYED_SENSE, 66);
        assert_eq!(UHF_DEPLOY_OUT, 115);
        assert_eq!(UHF_DEPLOYED_SENSE, 7);
    }

    #[test]
    fn gpio_numbers_are_unique() {
        let mut seen = PIN_TABLE.map(|p| p.gpio);
        seen.sort_unstable();
        for pair in seen.windows(2) {
            assert_ne!(pair[0], pair[1], "duplicate GPIO number in PIN_TABLE");
        }
    }
}