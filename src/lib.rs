// Visca over IP library based on datasheet found at:
// https://www.sony.net/Products/CameraSystem/CA/BRC_X1000_BRC_H800/Technical_Document/C456100121.pdf

use std::{io, io::Read, net::UdpSocket, time::Duration};

use messages::{ViscaCommand, MessageType};

pub mod commands;
pub mod inquiry;
mod messages;

pub struct ViscaError {}
pub type Result<T> = std::result::Result<T, ViscaError>;


pub struct Camera {
  socket: UdpSocket,
  seqnum: u32,
  timeout: Option<Duration>,
}

impl Camera {
  pub fn new(addr: &str) -> io::Result<Self> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(addr)?;
    Ok(Camera { socket, seqnum: 0 , timeout: None})
  }

  pub fn set_timout(&mut self, timeout: Option<Duration>) {
    self.timeout = timeout;
  }
  
  pub fn reset_seqnum(&mut self) -> io::Result<()> {
    self.send_bytes(MessageType::ControlCommand, &[0x01])
}

  pub fn set(&self, command: impl ViscaCommand) -> io::Result<()> {
    todo!();
  }

  pub fn send_bytes(&mut self, message_type: MessageType, bytes: &[u8]) -> io::Result<()> {
    let payload_type: [u8; 2] = (message_type as u16).to_be_bytes();

    let payload_length = bytes.len() as u16;
    let payload_length = payload_length.to_be_bytes();
    let seq: [u8; 4] = self.seqnum.to_be_bytes();

    // TODO: simplify commands by using redundancies
    // payload for all commands begins with 0x81, {0x01 or 0x04}
    // payload for all inquiries begins with 0x09, {0x01 or 0x04}

    let message = [
      &payload_type,
      &payload_length,
      &seq[0..=1],
      &seq[2..=3],
      bytes,
    ]
    .concat();

    self.socket.send(&message)?;
    self.seqnum = self.seqnum.wrapping_add(1);
    Ok(())
}

  pub fn scan(timout: u64) -> io::Result<Vec<String>> {
    let sock = UdpSocket::bind("0.0.0.0:52380")?;
    sock.set_broadcast(true)?;
    sock.set_read_timeout(Some(Duration::from_millis(timout)))?;
    let inq = [&[0x02], "ENQ:network".as_bytes(), &[0xFF, 0x03]].concat();
    sock.send_to(&inq, "255.255.255.255:52380")?;

    println!("Scanning for cameras...\n");

    let mut output = Vec::new();
    
    let mut buf = [0u8; 1500];
    while let Ok(received) = sock.recv(&mut buf) {
      let received = buf[..received]
        .bytes()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

        // if the received packet contains the text 'MAC' (it's a response to the query)
        //   then extract the ascii and print it.
        if vec_contains_slice(&received, "MAC".as_bytes()).is_some() {
          let mut camreply = String::new();
          for elem in received[1..received.len() - 1]
            .split(|byte| *byte == 0xFFu8)
            .filter(|byte| byte.is_ascii())
          {
            if let Ok(text) = String::from_utf8(elem.to_vec()){
              camreply.push_str(&format!("{text}"));
            }
          }
          output.push(camreply);
        }
      }
    Ok(output)
  }
}

fn vec_contains_slice<T: PartialEq>(v: &[T], s: &[T]) -> Option<usize> {
  if v.len() < s.len() {
    return None;
  };

  'outer: for i in 0..=(v.len() - s.len()) {
    for j in 0..s.len() {
      if v[i + j] != s[j] {
        continue 'outer;
      };
    }
    return Some(i);
  }
  None
}