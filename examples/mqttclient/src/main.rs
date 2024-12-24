#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use core::time::Duration;
use std::io::{self, prelude::*};
use std::net::{TcpStream, ToSocketAddrs};

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

    if let Packet::ConnAck(ack) = res {
        println!("connack: {:?}", ack);
    } else {
        panic!("invalid connack");
    }

    Ok(())
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, simple http client!");
    client().expect("test http client failed");
}
