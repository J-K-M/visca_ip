#![allow(dead_code)]

#[derive(Debug, PartialEq)]
#[repr(u16)]
pub enum MessageType {
    Command = 0x0100u16,
    Inquiry = 0x0110,
    Reply = 0x0111,
    DevSettingCommand = 0x0120,
    ControlCommand = 0x0200,
    ControlReply = 0x0201,
}

impl MessageType {
    pub fn to_be_bytes(self) -> [u8; 2] {
        (self as u16).to_be_bytes()
    }
}

pub trait ViscaMessage {
    fn bytes(&self) -> Vec<u8>;
    fn msg_type(&self) -> MessageType;
}

pub trait ViscaInquiry : ViscaMessage {
    fn parse_reply(&self, bytes: &[u8]) -> String;
}

pub trait ViscaCommand : ViscaMessage {
    fn parse_reply(&self, bytes: &[u8]) -> String;
}

// Util functions to extract nibbles from u8/16/32
pub fn u8top(val: u8) -> u8 {
    (val & 0xF0) >> 4
}
pub fn u8bot(val: u8) -> u8 {
    val & 0x0F
}

pub fn u16top(val: u16) -> u8 {
    ((val & 0xF000) >> 12) as u8
}

pub fn u16midtop(val: u16) -> u8 {
    ((val & 0x0F00) >> 8) as u8
}

pub fn u16midbot(val: u16) -> u8 {
    ((val & 0x00F0) >> 4) as u8
}

pub fn u16bot(val: u16) -> u8 {
    (val & 0x000F) as u8
}

pub fn u32byte8(val: u32) -> u8 {
    // most significant nibble
    ((val & 0xF000_0000) >> 28) as u8
}
pub fn u32byte6(val: u32) -> u8 {
    ((val & 0x0F00_0000) >> 24) as u8
}
pub fn u32byte5(val: u32) -> u8 {
    ((val & 0x00F0_0000) >> 20) as u8
}
pub fn u32byte4(val: u32) -> u8 {
    ((val & 0x000F_0000) >> 16) as u8
}
pub fn u32byte3(val: u32) -> u8 {
    ((val & 0x0000_F000) >> 12) as u8
}
pub fn u32byte2(val: u32) -> u8 {
    ((val & 0x0000_0F00) >> 8) as u8
}
pub fn u32byte1(val: u32) -> u8 {
    ((val & 0x0000_00F0) >> 4) as u8
}
pub fn u32byte0(val: u32) -> u8 {
    // least significant nibble
    (val & 0x0000_000F) as u8
}


pub fn merge_u8(top: u8, bot: u8) -> u8 {
    (top << 4) | bot
}

pub fn merge_u16(top: u8, midtop: u8, midbot: u8, bot: u8) -> u16 {
    let top = top as u16;
    let midtop = midtop as u16;
    let midbot = midbot as u16;
    let bot = bot as u16;
    (top << 12) | (midtop << 8) | (midbot << 4) | bot
}