#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

#[cfg(feature = "axstd")]
extern crate alloc;

use alloc::string::String;
use core::time::Duration;
use mqtt_lite::*;
use std::io::BufRead;
use std::io::{self, prelude::*};
use std::net::{TcpStream, ToSocketAddrs};
use std::thread::{sleep, spawn};

#[cfg(feature = "dns")]
const DEST: &str = "ident.me:80";
#[cfg(not(feature = "dns"))]
const DEST: &str = "10.0.0.110:1883";

const LF: u8 = b'\n';
const CR: u8 = b'\r';
const DL: u8 = b'\x7f';
const BS: u8 = b'\x08';
const SPACE: u8 = b' ';

const MAX_CMD_LEN: usize = 256;

struct Socket(TcpStream);

impl mqtt_lite::BufRead for Socket {
    fn read_exact(&mut self, buff: &mut [u8]) -> Result<(), MqttError> {
        self.0
            .read_exact(buff)
            .map_err(|_e| MqttError::Disconnected)
    }
}

fn client() -> io::Result<()> {
    for addr in DEST.to_socket_addrs()? {
        println!("dest: {} ({})", DEST, addr);
    }

    let mut stream = TcpStream::connect(DEST)?;

    let mut connect = Connect::new("arceos");

    connect.keep_alive = Duration::from_secs(65535);

    let req = Packet::Connect(connect);

    let connect_bytes = req.to_bytes();

    stream.write_all(&connect_bytes).unwrap();

    println!("connect send ok");

    let mut socket = Socket(stream);

    let res = Packet::read_from(&mut socket).expect("read connack fail");

    let ack = match res {
        Packet::ConnAck(ack) => ack,
        _ => panic!("invalid connack"),
    };

    println!("connack: {:?}", ack);

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut buf = [0; MAX_CMD_LEN];
    let mut cursor = 0;

    loop {
        if stdin.read(&mut buf[cursor..cursor + 1]).ok() != Some(1) {
            continue;
        }
        if buf[cursor] == b'\x1b' {
            buf[cursor] = b'^';
        }
        match buf[cursor] {
            CR | LF => {
                println!();
                if cursor > 0 {
                    let req = Packet::Publish {
                        dup: false,
                        qos: 0,
                        retain: false,
                        data: Publish::new("test1", Payload::Binary(buf[..cursor].to_vec()), None),
                    };

                    let bytes = req.to_bytes();

                    socket.0.write_all(&bytes).unwrap();
                    cursor = 0;
                }
            }
            BS | DL => {
                if cursor > 0 {
                    stdout.write_all(&[BS, SPACE, BS]).unwrap();
                    cursor -= 1;
                }
            }
            0..=31 => {}
            c => {
                if cursor < MAX_CMD_LEN - 1 {
                    stdout.write_all(&[c]).unwrap();
                    cursor += 1;
                }
            }
        }
    }
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, simple mqtt client!");
    client().expect("test http client failed");
}
