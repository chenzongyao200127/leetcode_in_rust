use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{self, Result};
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MessageType {
    M1 = 0,
    M2 = 1,
    M3 = 2,
    M4 = 3,
    M5 = 4,
}

type Priority = u8;

// Pre-define the priority mapping to avoid re-creating it every function call.
lazy_static! {
    static ref PRIORITIES: HashMap<MessageType, Priority> = HashMap::from([
        (MessageType::M1, 0),
        (MessageType::M2, 1),
        (MessageType::M3, 2),
        (MessageType::M4, 3),
        (MessageType::M5, 4),
    ]);
}

fn get_priority(msg_type: MessageType) -> Option<Priority> {
    PRIORITIES.get(&msg_type).cloned()
}

struct Message {
    msg_type: MessageType,
    data: Vec<u8>,
}

struct UdpPacket {
    buffer: Vec<u8>,
}

impl UdpPacket {
    fn new() -> Self {
        UdpPacket { buffer: Vec::new() }
    }

    fn pack_messages(&mut self, messages: &[Message]) {
        for message in messages {
            if get_priority(message.msg_type).is_some() {
                // Push the message type as a byte
                self.buffer.push(message.msg_type as u8);
                // Push the message size as two bytes (big endian)
                let size = message.data.len() as u16;
                self.buffer.extend_from_slice(&size.to_be_bytes());
                // Extend the buffer with the message data
                self.buffer.extend_from_slice(&message.data);
            }
        }
    }
}

fn send(socket: &UdpSocket, addr: &SocketAddr, packet: &UdpPacket) -> Result<usize> {
    socket.send_to(&packet.buffer, addr)
}

fn receive(socket: &UdpSocket) -> Result<(UdpPacket, SocketAddr)> {
    let mut buffer = [0u8; 65507]; // Maximum size of a UDP packet
    let (size, src) = socket.recv_from(&mut buffer)?;
    Ok((
        UdpPacket {
            buffer: buffer[..size].to_vec(),
        },
        src,
    ))
}

fn main() -> Result<()> {
    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();

    // Prepare a list of messages
    let messages = vec![
        Message {
            msg_type: MessageType::M3,
            data: vec![1, 2, 3],
        },
        Message {
            msg_type: MessageType::M1,
            data: vec![4, 5, 6],
        },
    ];

    // Pack the messages into a UDP packet
    let mut packet = UdpPacket::new();
    packet.pack_messages(&messages);

    // Send the UDP packet
    send(&socket, &addr, &packet)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority(MessageType::M1), Some(0));
        assert_eq!(get_priority(MessageType::M5), Some(4));
        assert!(get_priority(MessageType::M1).is_some());
    }

    #[test]
    fn test_pack_messages() {
        let messages = vec![
            Message {
                msg_type: MessageType::M3,
                data: vec![1, 2, 3],
            },
            Message {
                msg_type: MessageType::M1,
                data: vec![4, 5, 6],
            },
        ];

        let mut packet = UdpPacket::new();
        packet.pack_messages(&messages);

        // Check that the buffer is not empty and has the expected length
        assert!(!packet.buffer.is_empty());
        // Since we are pushing 1 byte for the message type, 2 bytes for the size, and 3 bytes for the data
        // We expect the buffer to be of length 6 (1+2+3) for each message, hence 12 for two messages.
        assert_eq!(packet.buffer.len(), 12);
    }
}
