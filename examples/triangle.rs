extern crate visca_ip;
use visca_ip::*;
use std::{thread::sleep, time::Duration};

fn main() {
  let cam = Camera::new("10.0.0.1").unwrap();

  let xs: [u32; 3] = [0x0374A, 0, 0xFC8B6];
  let ys: [u16; 3] = [0, 0x374A, 0];
  let speed: u8 = 10;
  let mut i = 0;
  while let Ok(pos) = cam.set(commands::PanTilt::AbsolutePos(speed, xs[i], ys[i])) {
    println!("Camera was moved to {pos:?}.");
    i = (i + 1) % 2;
    sleep(Duration::from_secs(2));
  }
}