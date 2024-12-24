#[cfg(test)]
mod test {
    use std::{
        io::{Read, Write as _},
        net::TcpStream,
        time::Duration,
    };

    use mqtt_lite::*;

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

    #[test]
    fn test_connect() {
        let mut stream = TcpStream::connect(DEST).unwrap();

        let mut connect = Connect::new("arceos");

        connect.keep_alive = Duration::from_secs(10);

        let req = Packet::Connect(connect);

        let connect_bytes = req.to_bytes();

        println!("{:?}", connect_bytes);

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
    }
}
