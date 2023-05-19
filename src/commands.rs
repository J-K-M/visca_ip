#![allow(dead_code)]

use crate::messages::{self as msg, ViscaCommand};

/*
* TODO: Unify on/off behaviour
*/

#[derive(Debug)]
pub enum Exposure {
    ModeFullAuto,
    ModeManual,
    ModeShutterPri,
    ModeIrisPri,
    ModeGainPri,
    IrisReset,
    IrisUp,
    IrisDown,
    IrisDirect(u8), // Iris Position 05 - 15
    GainReset,
    GainUp,
    GainDown,
    GainDirect(u8),   // 00 (-3dB) - 0C (33dB)
    GainLimit(u8),    // 4 (9dB) - 9 (24dB), FF (off)
    GainPoint(u8),    // 2=on, 3=off
    GainPointPos(u8), // 01 (0dB) - 09 (24dB)
    ShutterReset,
    ShutterUp,
    ShutterDown,
    ShutterDirect(u8), // Shutter Position 01 - 15
    MaxShutter(u8),    // 03 - 15
    MinShutter(u8),    // 03 - 15
    AESpeed(u8),       // 01 - 30
    ExpCompOnOff(u8),  // 2=on, 3=off
    ExpCompReset,
    ExpCompUp,
    ExpCompDown,
    ExpCompDirect(u8), // 00 - 0E
    BackLight(u8),     // 2=on, 3=off
    SpotLight(u8),     // 2=on, 3=off
    VisEnhanceOn,
    VisEnhanceOff,
    VisEnhanceDirect(u8, u8, u8), /* p: Effect level:
                                   *    0 (Dark) - 6 (Bright)
                                   * q: Brightness compensation selection:
                                   *    0 (Very dark), 1 (Dark), 2 (Standard), 3 (Bright)
                                   * r: Compensation level:
                                   *    0 (Low), 1 (Mid), 2 (High) */

    IrCutFilterOnOff(u8),              // 2=on (night), 3=off (day)
    LowLightBasisBrightnessOnOff(u8),  // 2=on, 3=off
    LowLightBasisBrightnessDirect(u8), // 4 - A
    NDFilter(u8),                      // 0=off, 1=1/4, 2=1/16, 3=1/64
}

impl msg::ViscaMessage for Exposure {
    fn bytes(&self) -> Vec<u8> {
        match self {
            Self::ModeFullAuto => vec![0x81, 0x01, 0x04, 0x39, 0x00, 0xFF],
            Self::ModeManual => vec![0x81, 0x01, 0x04, 0x39, 0x03, 0xFF],
            Self::ModeShutterPri => vec![0x81, 0x01, 0x04, 0x39, 0x0A, 0xFF],
            Self::ModeIrisPri => vec![0x81, 0x01, 0x04, 0x39, 0x0B, 0xFF],
            Self::ModeGainPri => vec![0x81, 0x01, 0x04, 0x39, 0x0E, 0xFF],

            Self::IrisReset => vec![0x81, 0x01, 0x04, 0x0B, 0x00, 0xFF],
            Self::IrisUp => vec![0x81, 0x01, 0x04, 0x0B, 0x02, 0xFF],
            Self::IrisDown => vec![0x81, 0x01, 0x04, 0x0B, 0x03, 0xFF],
            Self::IrisDirect(val) => {
                vec![
                    0x81,
                    0x01,
                    0x04,
                    0x4B,
                    0x00,
                    0x00,
                    msg::u8top(*val),
                    msg::u8bot(*val),
                    0xFF,
                ]
            }

            Self::GainReset => vec![0x81, 0x01, 0x04, 0x0C, 0x00, 0xFF],
            Self::GainUp => vec![0x81, 0x01, 0x04, 0x0C, 0x02, 0xFF],
            Self::GainDown => vec![0x81, 0x01, 0x04, 0x0C, 0x03, 0xFF],
            Self::GainDirect(val) => {
                vec![
                    0x81,
                    0x01,
                    0x04,
                    0x4C,
                    0x00,
                    0x00,
                    msg::u8top(*val),
                    msg::u8bot(*val),
                    0xFF,
                ]
            }
            Self::GainLimit(val) => vec![0x81, 0x01, 0x04, 0x2C, *val, 0xFF],
            Self::GainPoint(val) => vec![0x81, 0x01, 0x05, 0x0C, *val, 0xFF],
            Self::GainPointPos(val) => vec![0x81, 0x01, 0x05, 0x4C, msg::u8top(*val), msg::u8bot(*val), 0xFF],

            Self::ShutterReset => vec![0x81, 0x01, 0x04, 0x0A, 0x00, 0xFF],
            Self::ShutterUp => vec![0x81, 0x01, 0x04, 0x0A, 0x02, 0xFF],
            Self::ShutterDown => vec![0x81, 0x01, 0x04, 0x0A, 0x03, 0xFF],
            Self::ShutterDirect(val) => {
                vec![
                    0x81,
                    0x01,
                    0x04,
                    0x4A,
                    0x00,
                    0x00,
                    msg::u8top(*val),
                    msg::u8bot(*val),
                    0xFF,
                ]
            }
            Self::MaxShutter(val) => {
                vec![0x81, 0x01, 0x05, 0x2A, 0x00, msg::u8top(*val), msg::u8bot(*val), 0xFF]
            }
            Self::MinShutter(val) => {
                vec![0x81, 0x01, 0x05, 0x2A, 0x01, msg::u8top(*val), msg::u8bot(*val), 0xFF]
            }

            Self::AESpeed(val) => vec![0x81, 0x01, 0x04, 0x5D, *val, 0xFF],

            Self::ExpCompOnOff(val) => vec![0x81, 0x01, 0x04, 0x3E, *val, 0xFF],
            Self::ExpCompReset => vec![0x81, 0x01, 0x04, 0x0E, 0x00, 0xFF],
            Self::ExpCompUp => vec![0x81, 0x01, 0x04, 0x0E, 0x02, 0xFF],
            Self::ExpCompDown => vec![0x81, 0x01, 0x04, 0x0E, 0x03, 0xFF],
            Self::ExpCompDirect(val) => {
                vec![
                    0x81,
                    0x01,
                    0x04,
                    0x4E,
                    0x00,
                    0x00,
                    msg::u8top(*val),
                    msg::u8bot(*val),
                    0xFF,
                ]
            }

            Self::BackLight(val) => vec![0x81, 0x01, 0x04, 0x33, *val, 0xFF],

            Self::SpotLight(val) => vec![0x81, 0x01, 0x04, 0x3A, *val, 0xFF],

            // not standard! on = 06, off = 03
            Self::VisEnhanceOn => vec![0x81, 0x01, 0x04, 0x3D, 0x06, 0xFF],
            Self::VisEnhanceOff => vec![0x81, 0x01, 0x04, 0x3D, 0x03, 0xFF],
            Self::VisEnhanceDirect(p, q, r) => vec![
                0x81, 0x01, 0x04, 0x2D, 0x00, *p, *q, *r, 0x00, 0x00, 0x00, 0x00, 0xFF,
            ],

            Self::IrCutFilterOnOff(val) => vec![0x81, 0x01, 0x04, 0x01, *val, 0xFF],

            Self::LowLightBasisBrightnessOnOff(val) => vec![0x81, 0x01, 0x05, 0x39, *val, 0xFF],
            Self::LowLightBasisBrightnessDirect(val) => vec![0x81, 0x01, 0x05, 0x49, *val, 0xFF],

            Self::NDFilter(val) => vec![0x81, 0x01, 0x7E, 0x01, 0x53, *val, 0xFF],
        }
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for Exposure {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum Colour {
    WBAuto1,
    WBIndoor,
    WBOutdoor,
    WBOnePush,
    WBAuto2,
    WBManual,
    OnePushTrigger,
    RGainReset,
    RGainUp,
    RGainDown,
    RGainDirect(u8), // 00 (-128) - 80 (0) - FF (128)
    BGainReset,
    BGainUp,
    BGainDown,
    BGainDirect(u8), // 00 (-128) - 80 (0) - FF (128)
    Speed(u8),       // 1 (slow) - 5 (fast)
    OffsetReset,
    OffsetUp,
    OffsetDown,
    OffsetDirect(u8),   // 0 (-7) - 7 (0) - E (+7)
    ChromaSuppress(u8), // 0 (off), 1 (weak) - 3 (strong)
    MatrixSelect(u8),   /* Matrix Setting (2=STD, 3=OFF, 4=HIGH SAT,
                         * 5=FL LIGHT, 6=MOVIE, 7=STILL,
                         * 8=CINEMA, 9=PRO, A=ITU709, B=B/W) */
    LevelReset,
    LevelUp,
    LevelDown,
    LevelDirect(u8), // 0 (0) - E (14)
    PhaseReset,
    PhaseUp,
    PhaseDown,
    PhaseDirect(u8), // 0 (-14 degrees) - E (+14 degrees)
    RG(u8),          //00 (–99) - 63 (00) - C6 (+99)
    RB(u8),          //00 (–99) - 63 (00) - C6 (+99)
    GR(u8),          //00 (–99) - 63 (00) - C6 (+99)
    GB(u8),          //00 (–99) - 63 (00) - C6 (+99)
    BR(u8),          //00 (–99) - 63 (00) - C6 (+99)
    BG(u8),          //00 (–99) - 63 (00) - C6 (+99)
}

impl msg::ViscaMessage for Colour {
    fn bytes(&self) -> Vec<u8> {
        match self {
            Self::WBAuto1 => vec![0x81, 0x01, 0x04, 0x35, 0x00, 0xFF],
            Self::WBIndoor => vec![0x81, 0x01, 0x04, 0x35, 0x01, 0xFF],
            Self::WBOutdoor => vec![0x81, 0x01, 0x04, 0x35, 0x02, 0xFF],
            Self::WBOnePush => vec![0x81, 0x01, 0x04, 0x35, 0x03, 0xFF],
            Self::WBAuto2 => vec![0x81, 0x01, 0x04, 0x35, 0x04, 0xFF],
            Self::WBManual => vec![0x81, 0x01, 0x04, 0x35, 0x05, 0xFF],
            Self::OnePushTrigger => vec![0x81, 0x01, 0x04, 0x10, 0x05, 0xFF],

            Self::RGainReset => vec![0x81, 0x01, 0x04, 0x03, 0x00, 0xFF],
            Self::RGainUp => vec![0x81, 0x01, 0x04, 0x03, 0x02, 0xFF],
            Self::RGainDown => vec![0x81, 0x01, 0x04, 0x03, 0x03, 0xFF],
            Self::RGainDirect(val) => {
                vec![
                    0x81,
                    0x01,
                    0x04,
                    0x43,
                    0x00,
                    0x00,
                    msg::u8top(*val),
                    msg::u8bot(*val),
                    0xFF,
                ]
            }
            Self::BGainReset => vec![0x81, 0x01, 0x04, 0x04, 0x00, 0xFF],
            Self::BGainUp => vec![0x81, 0x01, 0x04, 0x04, 0x02, 0xFF],
            Self::BGainDown => vec![0x81, 0x01, 0x04, 0x04, 0x03, 0xFF],
            Self::BGainDirect(val) => {
                vec![
                    0x81,
                    0x01,
                    0x04,
                    0x44,
                    0x00,
                    0x00,
                    msg::u8top(*val),
                    msg::u8bot(*val),
                    0xFF,
                ]
            }
            Self::Speed(val) => vec![0x81, 0x01, 0x04, 0x56, *val, 0xFF],

            Self::OffsetReset => vec![0x81, 0x01, 0x7E, 0x01, 0x2E, 0x00, 0x00, 0xFF],
            Self::OffsetUp => vec![0x81, 0x01, 0x7E, 0x01, 0x2E, 0x00, 0x02, 0xFF],
            Self::OffsetDown => vec![0x81, 0x01, 0x7E, 0x01, 0x2E, 0x00, 0x03, 0xFF],
            Self::OffsetDirect(val) => vec![0x81, 0x01, 0x7E, 0x01, 0x2E, 0x01, *val, 0xFF],

            Self::ChromaSuppress(val) => vec![0x81, 0x01, 0x04, 0x5F, *val, 0xFF],
            Self::MatrixSelect(val) => vec![0x81, 0x01, 0x7E, 0x01, 0x3D, *val, 0xFF],

            Self::LevelReset => vec![0x81, 0x01, 0x04, 0x09, 0x00, 0xFF],
            Self::LevelUp => vec![0x81, 0x01, 0x04, 0x09, 0x02, 0xFF],
            Self::LevelDown => vec![0x81, 0x01, 0x04, 0x09, 0x03, 0xFF],
            Self::LevelDirect(val) => vec![0x81, 0x01, 0x04, 0x49, 0x00, 0x00, 0x00, *val, 0xFF],

            Self::PhaseReset => vec![0x81, 0x01, 0x04, 0x0F, 0x00, 0xFF],
            Self::PhaseUp => vec![0x81, 0x01, 0x04, 0x0F, 0x02, 0xFF],
            Self::PhaseDown => vec![0x81, 0x01, 0x04, 0x0F, 0x03, 0xFF],
            Self::PhaseDirect(val) => vec![0x81, 0x01, 0x04, 0x4F, 0x00, 0x00, 0x00, *val, 0xFF],

            Self::RG(val) => vec![0x81, 0x01, 0x7E, 0x01, 0x7A, msg::u8top(*val), msg::u8bot(*val), 0xFF],
            Self::RB(val) => vec![0x81, 0x01, 0x7E, 0x01, 0x7B, msg::u8top(*val), msg::u8bot(*val), 0xFF],
            Self::GR(val) => vec![0x81, 0x01, 0x7E, 0x01, 0x7C, msg::u8top(*val), msg::u8bot(*val), 0xFF],
            Self::GB(val) => vec![0x81, 0x01, 0x7E, 0x01, 0x7D, msg::u8top(*val), msg::u8bot(*val), 0xFF],
            Self::BR(val) => vec![0x81, 0x01, 0x7E, 0x01, 0x7E, msg::u8top(*val), msg::u8bot(*val), 0xFF],
            Self::BG(val) => vec![0x81, 0x01, 0x7E, 0x01, 0x7F, msg::u8top(*val), msg::u8bot(*val), 0xFF],
        }
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for Colour {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum Detail {
    LevelReset,
    LevelUp,
    LevelDown,
    LevelDirect(u8),
    Mode(u8),
    Bandwidth(u8),
    Crispening(u8),
    HVBalance(u8),
    BWBalance(u8),
    Limit(u8),
    HighlightedTail(u8),
    Superlow(u8),
}

impl msg::ViscaMessage for Detail {
    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for Detail {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum Knee {
    SettingOnOff,
    Mode(u8),
    Slope(u8),
    Point(u8),
}

impl msg::ViscaMessage for Knee {
    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for Knee {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum Gamma {
    Mode(u8),
    Pattern(u8),
    Offset(u8),
    Level(u8),
    BlackGammaLevel(u8),
    BlackGammaRange(u8),
    BlackLevelReset,
    BlackLevelUp,
    BlackLevelDown,
    BlackLevelDirect(u8),
}

impl msg::ViscaMessage for Gamma {
    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for Gamma {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum PictureProfile {
    Mode(u8),
}

impl msg::ViscaMessage for PictureProfile {
    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for PictureProfile {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum FlickerReduction {
    Mode(u8),
}

impl msg::ViscaMessage for FlickerReduction {
    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for FlickerReduction {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum NoiseReduction {
    ModeLevel(u8),
    NR2d3dSetting(u8),
}

impl msg::ViscaMessage for NoiseReduction {
    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for NoiseReduction {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum Zoom {
    Stop,
    TeleStd,
    WideStd,
    TeleVar(u8), // 0 (low) - 7 (high)
    WideVar(u8), // 0 (low) - 7 (high)
    Direct(u16), /* 0000 (wide) to 4000 (optical tele)
                  * to 5580 (Clear Image Zoom tele 4K)
                  * 0000 (wide) to 4000 (optical tele)
                  * to 6000 (Clear Image Zoom tele FHD) */
    ClearImageZoomOff,
    ClearImageZoomOn,
    TeleConvMode(u8), /* 2=double, 3=off
                       * (Cannot be used when the signal
                       * format is other than 1080/29.97p,
                       * 1080/25p, or 1080/23.98p) */
}

impl msg::ViscaMessage for Zoom {
    fn bytes(&self) -> Vec<u8> {
        match self {
            Self::Stop => vec![0x81, 0x01, 0x04, 0x07, 0x00, 0xFF],
            Self::TeleStd => vec![0x81, 0x01, 0x04, 0x07, 0x02, 0xFF],
            Self::WideStd => vec![0x81, 0x01, 0x04, 0x07, 0x03, 0xFF],
            Self::TeleVar(val) => vec![0x81, 0x01, 0x04, 0x07, (0x20 | msg::u8bot(*val)), 0xFF],
            Self::WideVar(val) => vec![0x81, 0x01, 0x04, 0x07, (0x30 | msg::u8bot(*val)), 0xFF],
            Self::Direct(val) => vec![
                0x81,
                0x01,
                0x04,
                0x47,
                msg::u16top(*val),
                msg::u16midtop(*val),
                msg::u16midbot(*val),
                msg::u16bot(*val),
                0xFF,
            ],
            Self::ClearImageZoomOff => vec![0x81, 0x01, 0x04, 0x06, 0x03, 0xFF],
            Self::ClearImageZoomOn => vec![0x81, 0x01, 0x04, 0x06, 0x04, 0xFF],
            Self::TeleConvMode(val) => vec![0x81, 0x01, 0x7E, 0x04, 0x36, *val, 0xFF],
        }
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for Zoom {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum Focus {
    Auto,
    Manual,
    AutoManualToggle,
    Stop,
    FarStdSpeed,
    NearStdSpeed,
    FarVarSpeed(u8),  // 0 (low) - 7 (high)
    NearVarSpeed(u8), // 0 (low) - 7 (high)
    Direct(u16),      // F000 (near) - 0000 (far)
    OnePushTrig,
    FocusInf,
    NearLimit(u16),    // 1000 - F000
    AFSensitivity(u8), // 2 (normal), 3 (low)
    IRCorrection(u8),  // 0 (standard), 1 (IR light)
}

impl msg::ViscaMessage for Focus {
    fn bytes(&self) -> Vec<u8> {
        match self {
            Focus::Auto => vec![0x81, 0x01, 0x04, 0x38, 0x02, 0xFF],
            Focus::Manual => vec![0x81, 0x01, 0x04, 0x38, 0x03, 0xFF],
            Focus::AutoManualToggle => vec![0x81, 0x01, 0x04, 0x38, 0x10, 0xFF],
            Focus::Stop => vec![0x81, 0x01, 0x04, 0x08, 0x00, 0xFF],
            Focus::FarStdSpeed => vec![0x81, 0x01, 0x04, 0x08, 0x02, 0xFF],
            Focus::NearStdSpeed => vec![0x81, 0x01, 0x04, 0x08, 0x03, 0xFF],
            Focus::FarVarSpeed(p) => vec![0x81, 0x01, 0x04, 0x08, (0x20 | msg::u8bot(*p)), 0xFF],
            Focus::NearVarSpeed(p) => vec![0x81, 0x01, 0x04, 0x08, (0x30 | msg::u8bot(*p)), 0xFF],
            Focus::Direct(pppp) => vec![
                0x81,
                0x01,
                0x04,
                0x48,
                msg::u16top(*pppp),
                msg::u16midtop(*pppp),
                msg::u16midbot(*pppp),
                msg::u16bot(*pppp),
                0xFF,
            ],
            Focus::OnePushTrig => vec![0x81, 0x01, 0x04, 0x18, 0x01, 0xFF],
            Focus::FocusInf => vec![0x81, 0x01, 0x04, 0x18, 0x02, 0xFF],
            Focus::NearLimit(pppp) => vec![
                0x81,
                0x01,
                0x04,
                0x28,
                msg::u16top(*pppp),
                msg::u16midtop(*pppp),
                msg::u16midbot(*pppp),
                msg::u16bot(*pppp),
                0xFF,
            ],
            Focus::AFSensitivity(p) => vec![0x81, 0x01, 0x04, 0x58, *p, 0xFF],
            Focus::IRCorrection(p) => vec![0x81, 0x01, 0x04, 0x11, *p, 0xFF],
        }
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for Focus {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum PanAngle {
    /* TODO THIS NEEDS TO BE FIXED
     * THE VALUE IS CONTINUOUS
     * these points are for reference only */
    Centre = 0x00000,
    Left10 = 0x00937,
    Left20 = 0x0126E,
    Left30 = 0x01BA5,
    Left40 = 0x024DC,
    Left50 = 0x02E13,
    Left60 = 0x0374A,
    Left70 = 0x04081,
    Left80 = 0x049B8,
    Left90 = 0x052EF,
    Left100 = 0x05C26,
    Left110 = 0x0655D,
    Left120 = 0x06E94,
    Left130 = 0x077CB,
    Left140 = 0x08102,
    Left150 = 0x08A39,
    Left160 = 0x09370,
    Left169 = 0x09BBB,
    Left170 = 0x09CA7,

    Right10 = 0xFF6C9,
    Right20 = 0xFED92,
    Right30 = 0xFE45B,
    Right40 = 0xFDB24,
    Right50 = 0xFD1ED,
    Right60 = 0xFC8B6,
    Right70 = 0xFBF7F,
    Right80 = 0xFB648,
    Right90 = 0xFAD11,
    Right100 = 0xFA3DA,
    Right110 = 0xF9AA3,
    Right120 = 0xF916C,
    Right130 = 0xF8835,
    Right140 = 0xF7EFE,
    Right150 = 0xF75C7,
    Right160 = 0xF6C90,
    Right169 = 0xF6445,
    Right170 = 0xF6359,
}

impl PanAngle {
    fn top(self) -> u8 {
        (((self as u32) & 0xF0000) >> 16) as u8
    }

    fn midtop(self) -> u8 {
        (((self as u32) & 0x0F000) >> 12) as u8
    }

    fn midmid(self) -> u8 {
        (((self as u32) & 0x00F00) >> 8) as u8
    }

    fn midbot(self) -> u8 {
        (((self as u32) & 0x000F0) >> 4) as u8
    }

    fn bot(self) -> u8 {
        ((self as u32) & 0x0000F) as u8
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum TiltAngle {
    /* TODO THIS NEEDS TO BE FIXED
     * THE VALUE IS CONTINUOUS
     * these points are for reference only */
    Centre = 0x0000,
    Up10 = 0x0937,
    Up20 = 0x126E,
    Up30 = 0x1BA5,
    Up40 = 0x24DC,
    Up50 = 0x2E13,
    Up60 = 0x374A,
    Up70 = 0x4081,
    Up80 = 0x49B8,
    Up90 = 0x52EF,

    Down10 = 0xF6C9,
    Down20 = 0xED92,
    Down30 = 0xE45B,
}

impl TiltAngle {
    fn top(self) -> u8 {
        msg::u16top(self as u16)
    }

    fn midtop(self) -> u8 {
        msg::u16midtop(self as u16)
    }

    fn midbot(self) -> u8 {
        msg::u16midbot(self as u16)
    }

    fn bot(self) -> u8 {
        msg::u16bot(self as u16)
    }
}

#[derive(Debug)]
pub enum PanTilt {
    Up(u8, u8),                // vv: Pan speed ww: Tilt speed 01 (Slow) - 18 (Fast)
    Down(u8, u8),              // vv: Pan speed ww: Tilt speed 01 (Slow) - 18 (Fast)
    Left(u8, u8),              // vv: Pan speed ww: Tilt speed 01 (Slow) - 18 (Fast)
    Right(u8, u8),             // vv: Pan speed ww: Tilt speed 01 (Slow) - 18 (Fast)
    UpLeft(u8, u8),            // vv: Pan speed ww: Tilt speed 01 (Slow) - 18 (Fast)
    UpRight(u8, u8),           // vv: Pan speed ww: Tilt speed 01 (Slow) - 18 (Fast)
    DownLeft(u8, u8),          // vv: Pan speed ww: Tilt speed 01 (Slow) - 18 (Fast)
    DownRight(u8, u8),         // vv: Pan speed ww: Tilt speed 01 (Slow) - 18 (Fast)
    Stop(u8, u8),              // vv: Pan speed ww: Tilt speed 01 (Slow) - 18 (Fast)
    AbsolutePos(u8, u32, u16), // vv: Speed 01 (Slow) - 18 (Fast) p: Pan angle t: tilt angle
    RelativePos(u8, u32, u16), // vv: Speed 01 (Slow) - 18 (Fast) p: Pan angle t: tilt angle
    Home,
    Reset,
    RampCurve(u8),          // p: 1 (Sharpness), 2 (Standard), 3 (Gentle)
    SlowMode(u8),           // p: 2=On, 3=Off
    LimitSet(u8, u32, u16), // w: Position (1=UpRight, 0=DownLeft) p: Pan position t: Tilt position
    LimitClear(u8),         // w: Position (1=UpRight, 0=DownLeft)
}

impl msg::ViscaMessage for PanTilt {
    fn bytes(&self) -> Vec<u8> {
        match self {
            Self::Up(vv, ww) => vec![0x81, 0x01, 0x06, 0x01, *vv, *ww, 0x03, 0x01, 0xFF],
            Self::Down(vv, ww) => vec![0x81, 0x01, 0x06, 0x01, *vv, *ww, 0x03, 0x02, 0xFF],
            Self::Left(vv, ww) => vec![0x81, 0x01, 0x06, 0x01, *vv, *ww, 0x01, 0x03, 0xFF],
            Self::Right(vv, ww) => vec![0x81, 0x01, 0x06, 0x01, *vv, *ww, 0x02, 0x03, 0xFF],
            Self::UpLeft(vv, ww) => vec![0x81, 0x01, 0x06, 0x01, *vv, *ww, 0x01, 0x01, 0xFF],
            Self::UpRight(vv, ww) => vec![0x81, 0x01, 0x06, 0x01, *vv, *ww, 0x02, 0x01, 0xFF],
            Self::DownLeft(vv, ww) => vec![0x81, 0x01, 0x06, 0x01, *vv, *ww, 0x01, 0x02, 0xFF],
            Self::DownRight(vv, ww) => vec![0x81, 0x01, 0x06, 0x01, *vv, *ww, 0x02, 0x02, 0xFF],
            Self::Stop(vv, ww) => vec![0x81, 0x01, 0x06, 0x01, *vv, *ww, 0x03, 0x03, 0xFF],
            Self::AbsolutePos(vv, p, t) => {
                vec![
                    0x81,
                    0x01,
                    0x06,
                    0x02,
                    *vv,
                    0x00,
                    msg::u32byte4(*p),
                    msg::u32byte3(*p),
                    msg::u32byte2(*p),
                    msg::u32byte1(*p),
                    msg::u32byte0(*p),
                    msg::u16top(*t),
                    msg::u16midtop(*t),
                    msg::u16midbot(*t),
                    msg::u16bot(*t),
                    0xFF,
                ]
            }
            Self::RelativePos(vv, p, t) => {
                vec![
                    0x81,
                    0x01,
                    0x06,
                    0x03,
                    *vv,
                    0x00,
                    msg::u32byte4(*p),
                    msg::u32byte3(*p),
                    msg::u32byte2(*p),
                    msg::u32byte1(*p),
                    msg::u32byte0(*p),
                    msg::u16top(*t),
                    msg::u16midtop(*t),
                    msg::u16midbot(*t),
                    msg::u16bot(*t),
                    0xFF,
                ]
            }
            Self::Home => vec![0x81, 0x01, 0x06, 0x04, 0xFF],
            Self::Reset => vec![0x81, 0x01, 0x06, 0x05, 0xFF],
            Self::RampCurve(p) => vec![0x81, 0x01, 0x06, 0x31, *p, 0xFF],
            Self::SlowMode(p) => vec![0x81, 0x01, 0x06, 0x44, *p, 0xFF],
            Self::LimitSet(w, p, t) => {
                vec![
                    0x81,
                    0x01,
                    0x06,
                    0x07,
                    0x00,
                    *w,
                    msg::u32byte4(*p),
                    msg::u32byte3(*p),
                    msg::u32byte2(*p),
                    msg::u32byte1(*p),
                    msg::u32byte0(*p),
                    msg::u16top(*t),
                    msg::u16midtop(*t),
                    msg::u16midbot(*t),
                    msg::u16bot(*t),
                    0xFF,
                ]
            }
            Self::LimitClear(w) => vec![
                0x81, 0x01, 0x06, 0x07, 0x01, *w, 0x07, 0x0F, 0x0F, 0x0F, 0x0F, 0x07, 0x0F, 0x0F,
                0x0F, 0xFF,
            ],
        }
    }
    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for PanTilt {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum Preset {
    Reset(u8),  // memory number
    Set(u8),    // memory number
    Recall(u8), // memory number
    PresetDriveSpeed(u8, u8), /* pp: Preset number of speed setting –1 (00 - 63)
                 * qq: pp position direction speed 01 - 18 */
    PresetMode(u8), //00=MODE1, 01=MODE2, 10=TRACE
}

impl msg::ViscaMessage for Preset {
    fn bytes(&self) -> Vec<u8> {
        match self {
            Preset::Reset(p) => vec![0x81, 0x01, 0x04, 0x3F, 0x00, *p, 0xFF],
            Preset::Set(p) => vec![0x81, 0x01, 0x04, 0x3F, 0x01, *p, 0xFF],
            Preset::Recall(p) => vec![0x81, 0x01, 0x04, 0x3F, 0x02, *p, 0xFF],
            Preset::PresetDriveSpeed(pp, qq) => {
                vec![0x81, 0x01, 0x7E, 0x01, 0x0B, *pp, *qq, 0xFF]
            }
            Preset::PresetMode(pp) => vec![0x81, 0x01, 0x7E, 0x04, 0x3D, *pp, 0xFF],
        }
    }

    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for Preset {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum PTZTrace {
    RecStart(u8), // 0 - F (PTZ Trace number to record 1 - 16)}
    RecStop,
    PlayPrepare(u8), // 0 - F (PTZ Trace number to reproduce 1 - 16)
    PlayStart,
    Delete(u8), // 0 - F (PTZ Trace number to delete 1 - 16)
}

impl msg::ViscaMessage for PTZTrace {
    fn bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for PTZTrace {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum System {
    IRReceive(u8), // 02=on, 03=off, 10=toggle
    HPhaseUp,
    HPhaseDown,
    HPhaseDirect(u16),    // ppp: 000 - 3BF
    ImgFlipOnOff(u8),     // p: 2=On, 3=Off
    PanReverseOnOFf(u8),  // p: 1=ON, 0=OFF
    TiltReverseOnOff(u8), // p: 1=ON, 0=OFF
    MenuDisplayOff,
}

impl msg::ViscaMessage for System {
    fn bytes(&self) -> Vec<u8> {
        match self {
            System::IRReceive(p) => vec![0x81, 0x01, 0x06, 0x08, *p, 0xFF],
            System::HPhaseUp => vec![0x81, 0x01, 0x7E, 0x01, 0x3E, 0x00, 0x02, 0xFF],
            System::HPhaseDown => vec![0x81, 0x01, 0x7E, 0x01, 0x3E, 0x00, 0x03, 0xFF],
            System::HPhaseDirect(ppp) => vec![
                0x81,
                0x81,
                0x01,
                0x7E,
                0x01,
                0x5B,
                0x00,
                msg::u16midtop(*ppp),
                msg::u16midbot(*ppp),
                msg::u16bot(*ppp),
                0xFF,
            ],
            System::ImgFlipOnOff(p) => vec![0x81, 0x01, 0x04, 0x66, *p, 0xFF],
            System::PanReverseOnOFf(p) => vec![0x81, 0x01, 0x7E, 0x01, 0x06, 0x00, *p, 0xFF],
            System::TiltReverseOnOff(p) => vec![0x81, 0x01, 0x7E, 0x01, 0x09, 0x00, *p, 0xFF],
            System::MenuDisplayOff => vec![0x81, 0x01, 0x06, 0x06, 0x03, 0xFF],
        }
    }

    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for System {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub enum HDMI {}

impl msg::ViscaMessage for HDMI {
    fn bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn msg_type(&self) -> msg::MessageType {
        msg::MessageType::Command
    }
}

impl ViscaCommand for HDMI {
    fn parse_reply(&self, bytes: &[u8]) -> String {
        todo!()
    }
}