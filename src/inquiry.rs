#![allow(dead_code)]

use crate::messages::*;

#[derive(Debug)]
pub enum Exposure {
    Mode,
    Iris,
    Gain,
    GainLimit,
    GainPoint,
    GainPointPosition,
    Shutter,
    MaxShutter,
    MinShutter,
    AESpeed,
    ExpCompOnOff,
    ExpCompLevel,
    BackLight,
    SpotLight,
    VisEnhanceOnOff,
    VisEnhanceLevel,
    LowLightBasisBrightnessOnOff,
    LowLightBasisBrightnessLevel,
    NDFilter,
}

impl ViscaMessage for Exposure {
    fn bytes(&self) -> Vec<u8> {
        match self {
            Exposure::Mode => vec![0x81, 0x09, 0x04, 0x39, 0xFF],
            Exposure::Iris => vec![0x81, 0x09, 0x04, 0x4B, 0xFF],
            Exposure::Gain => vec![0x81, 0x09, 0x04, 0x4C, 0xFF],
            Exposure::GainLimit => vec![0x81, 0x09, 0x04, 0x2C, 0xFF],
            Exposure::GainPoint => vec![0x81, 0x09, 0x05, 0x0C, 0xFF],
            Exposure::GainPointPosition => vec![0x81, 0x09, 0x05, 0x4C, 0xFF],
            Exposure::Shutter => vec![0x81, 0x09, 0x04, 0x4A, 0xFF],
            Exposure::MaxShutter => vec![0x81, 0x09, 0x05, 0x2A, 0x00, 0xFF],
            Exposure::MinShutter => vec![0x81, 0x09, 0x05, 0x2A, 0x01, 0xFF],
            Exposure::AESpeed => vec![0x81, 0x09, 0x04, 0x5D, 0xFF],
            Exposure::ExpCompOnOff => vec![0x81, 0x09, 0x04, 0x3E, 0xFF],
            Exposure::ExpCompLevel => vec![0x81, 0x09, 0x04, 0x4E, 0xFF],
            Exposure::BackLight => vec![0x81, 0x09, 0x04, 0x33, 0xFF],
            Exposure::SpotLight => vec![0x81, 0x09, 0x04, 0x3A, 0xFF],
            Exposure::VisEnhanceOnOff => vec![0x81, 0x09, 0x04, 0x3D, 0xFF],
            Exposure::VisEnhanceLevel => vec![0x81, 0x09, 0x04, 0x2D, 0xFF],
            Exposure::LowLightBasisBrightnessOnOff => vec![0x81, 0x09, 0x05, 0x39, 0xFF],
            Exposure::LowLightBasisBrightnessLevel => vec![0x81, 0x09, 0x05, 0x49, 0xFF],
            Exposure::NDFilter => unimplemented!("VISCA Document has incorrect bytes"),
        }
    }

    fn msg_type(&self) -> MessageType {
        MessageType::Inquiry
    }
}


impl ViscaInquiry for Exposure {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        const SOCK: u8 = 0x90; // y0 at head of reply payload

        match self {
            Exposure::Mode => {
                match bytes {
                    [SOCK, 0x50, 0x00, 0xFF] => "Full Auto".to_string(),
                    [SOCK, 0x50, 0x03, 0xFF] => "Manual".to_string(),
                    [SOCK, 0x50, 0x0A, 0xFF] => "Shutter Priority".to_string(),
                    [SOCK, 0x50, 0x0B, 0xFF] => "Iris Priority".to_string(),
                    [SOCK, 0x50, 0x0E, 0xFF] => "Gain Priority".to_string(),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::Iris => {
                match bytes {
                    [SOCK, 0x50, 0x00, 0x00, p1, p2, 0xFF] => format!("Iris: {:02X}", merge_u8(*p1, *p2)),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::Gain => {
                match bytes {
                    [SOCK, 0x50, 0x00, 0x00, p1, p2, 0xFF] => format!("Gain: {:02X}", merge_u8(*p1, *p2)),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::GainLimit => {
                match bytes {
                    [SOCK, 0x50, p, 0xFF] => format!("Gain Limit: {:02X}", *p),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::GainPoint => {
                match bytes {
                    [SOCK, 0x50, 0x02, 0xFF] => "Gain Point ON".to_string(),
                    [SOCK, 0x50, 0x03, 0xFF] => "Gain Point OFF".to_string(),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::GainPointPosition => {
                match bytes {
                    [SOCK, 0x50, p1, p2, 0xFF] => format!("Gain Point Position: {:02X}", merge_u8(*p1, *p2)),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::Shutter => {
                match bytes {
                    [SOCK, 0x50, 0x00, 0x00, p1, p2, 0xFF] => format!("Shutter: {:02X}", merge_u8(*p1, *p2)),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::MaxShutter => {
                match bytes {
                    [SOCK, 0x50, p1, p2, 0xFF] => format!("Max Shutter: {:02X}", merge_u8(*p1, *p2)),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::MinShutter => {
                match bytes {
                    [SOCK, 0x50, p1, p2, 0xFF] => format!("Min Shutter: {:02X}", merge_u8(*p1, *p2)),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::AESpeed => {
                match bytes {
                    [SOCK, 0x50, p, 0xFF] => format!("AE Speed: {:02X}", *p),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::ExpCompOnOff => {
                match bytes {
                    [SOCK, 0x50, 0x02, 0xFF] => "Exposure Comp ON".to_string(),
                    [SOCK, 0x50, 0x03, 0xFF] => "Exposure Comp OFF".to_string(),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::ExpCompLevel => {
                match bytes {
                    [SOCK, 0x50, 0x00, 0x00, p1, p2, 0xFF] => format!("Exposure Comp Level: {:02X}", merge_u8(*p1, *p2)),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::BackLight => {
                match bytes {
                    [SOCK, 0x50, 0x02, 0xFF] => "Back Light ON".to_string(),
                    [SOCK, 0x50, 0x03, 0xFF] => "Back Light OFF".to_string(),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::SpotLight => {
                match bytes {
                    [SOCK, 0x50, 0x02, 0xFF] => "Spot Light ON".to_string(),
                    [SOCK, 0x50, 0x03, 0xFF] => "Spot Light OFF".to_string(),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::VisEnhanceOnOff => {
                match bytes {
                    [SOCK, 0x50, 0x06, 0xFF] => "Visibility Enhancer ON".to_string(),
                    [SOCK, 0x50, 0x03, 0xFF] => "Visibility Enhancer OFF".to_string(),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::VisEnhanceLevel => {
                match bytes {
                    [SOCK, 0x50, 0x00, p, q, r, 0x00, 0x00, 0x00, 0x00, 0xFF] => 
                        format!("Visibility Effect: Level {}, Brightness Comp Selection {}, Comp Level {}", *p, *q, *r),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::LowLightBasisBrightnessOnOff => {
                match bytes {
                    [SOCK, 0x50, 0x02, 0xFF] => "Low Light Basis Brightness ON".to_string(),
                    [SOCK, 0x50, 0x03, 0xFF] => "Low Light Basis Brightness OFF".to_string(),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::LowLightBasisBrightnessLevel => {
                match bytes {
                    [SOCK, 0x50, p, 0xFF] => format!("Low Light Basis Brightness: {:02X}", *p),
                    x => format!("Unknown reply: {:?}", x),
                }
            },
            Exposure::NDFilter => {
                unimplemented!("VISCA reference contains incorrect bytes");
            },
        }
    }
}