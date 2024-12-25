#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

#[cfg(feature = "axstd")]
extern crate alloc;

use core::time::Duration;
use std::io::{self, prelude::*};
use std::net::{TcpStream, ToSocketAddrs};
use alloc::string::String;

use mqtt_lite::*;

#[cfg(feature = "dns")]
const DEST: &str = "ident.me:80";
#[cfg(not(feature = "dns"))]
const DEST: &str = "10.0.0.110:1883";

struct StreamReadIter<'a> {
    stream: &'a mut TcpStream,
}

impl mqtt_lite::BufRead for StreamReadIter<'_> {
    fn read_exact(&mut self, buff: &mut [u8]) -> Result<(), MqttError> {
        self.stream
            .read_exact(buff)
            .map_err(|_e| MqttError::Disconnected)?;
        Ok(())
    }
}

fn client() -> io::Result<()> {
    for addr in DEST.to_socket_addrs()? {
        println!("dest: {} ({})", DEST, addr);
    }

    let mut stream = TcpStream::connect(DEST)?;

    let mut connect = Connect::new("arceos");

    connect.keep_alive = Duration::from_secs(10);

    let req = Packet::Connect(connect);

    let connect_bytes = req.to_bytes();

    stream.write_all(&connect_bytes).unwrap();

    let mut streamiter = StreamReadIter {
        stream: &mut stream,
    };

    let res = Packet::read_from(&mut streamiter).expect("read connack fail");

    let ack = match res {
        Packet::ConnAck(ack) => ack,
        _ => panic!("invalid connack"),
    };

    println!("connack: {:?}", ack);

    let req = Packet::Publish {
        dup: false,
        qos: 0,
        retain: false,
        data: Publish::new("test1", Payload::Text(String::from("测试")), None),
    };

    let bytes = req.to_bytes();

    stream.write_all(&bytes).unwrap();

    Ok(())
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, simple http client!");
    client().expect("test http client failed");
}
