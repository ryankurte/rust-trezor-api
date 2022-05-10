use std::fmt::Display;
use std::io::{ErrorKind};
use std::time::Duration;
use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};

use crate::{AvailableDevice, Model};

use super::{AvailableDeviceTransport, error::Error};
use super::protocol::{Link, ProtocolV2, Protocol};

#[derive(Debug)]
pub struct AvailableEmulatorTransport {
	pub addr: SocketAddrV4,
}

impl Display for AvailableEmulatorTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UDP {}", self.addr)
    }
}

/// Emulator
pub struct EmulatorLink {
    pub sock: UdpSocket,
}

impl Link for EmulatorLink {
    fn write_chunk(&mut self, chunk: Vec<u8>) -> Result<(), Error> {
        self.sock.send(&chunk)?;
        Ok(())
    }

    fn read_chunk(&mut self) -> Result<Vec<u8>, Error> {
        let mut buff = [0u8; 16 * 1024];

        let n = match self.sock.recv(&mut buff) {
            Ok(n) => n,
            Err(e) if e.kind() == ErrorKind::WouldBlock => return Err(Error::DeviceReadTimeout),
            Err(e) => return Err(e.into()),
        };

        Ok((buff[..n]).to_vec())
    }
}

/// An implementation of the Transport interface for UDP/Emulated devices
pub struct EmulatorTransport {
    protocol: ProtocolV2<EmulatorLink>
}


impl EmulatorTransport {

    /// Find devices using emulator transport
    pub fn find_devices(debug: bool) -> Result<Vec<AvailableDevice>, Error> {

        // Setup addresses to check
        // TODO: allow this to be configured in the EmulatorTransport
        let addrs = vec![
            SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 21326),
        ];

        // Bind a UDP port for discovery
        let s = UdpSocket::bind("127.0.0.1:0")?;
        s.set_read_timeout(Some(Duration::from_millis(500)))?;

        let mut devices = vec![];

        // Poll on possible ports
        for a in &addrs {
            // Send ping
            s.send_to(r#"PINGPING"#.as_bytes(), a)?;

            // Await pong
            let mut buff = vec![0u8; 16];

            let n = match s.recv(&mut buff) {
                Ok(n) => n,
                Err(e) if e.kind() == ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e.into()),
            };

            if n != 8 {
                debug!("Received {} bytes (expected {})", n, 8);
                continue;
            }

            if &buff[..n] != r#"PONGPONG"#.as_bytes() {
                debug!("Received invalid response: {:02x?}", &buff[..n]);
                    continue
            }

            debug!("Discovered trezor at UDP: {}", a);

            devices.push(AvailableDevice{
                model: Model::Emulator,
                debug,
                transport: AvailableDeviceTransport::Udp(
                    AvailableEmulatorTransport{
                        addr: a.clone()
                    }
                ),
            })
        }


        Ok(devices)
    }

    /// Connect to device using emulator transport
    pub fn connect(device: &AvailableDevice) -> Result<Box<dyn super::Transport>, Error> {

        // Check transports match
        let transport = match device.transport {
			AvailableDeviceTransport::Udp(ref t) => t,
			_ => panic!("passed wrong AvailableDevice in EmulatorTransport::connect"),
		};

        // Bind new socket for device comms
        let sock = UdpSocket::bind("127.0.0.1:0")?;

        // Set endpoint address and timeouts
        // TODO: how does the library deal with async?
        sock.connect(transport.addr)?;

        let link = EmulatorLink{ sock };

        let t = EmulatorTransport{
            protocol: ProtocolV2{
                session_id: 0,
                link
            },
        };

        Ok(Box::new(t))
    }
}

impl super::Transport for EmulatorTransport {
    fn session_begin(&mut self) -> Result<(), Error> {
        self.protocol.session_begin()
    }

    fn session_end(&mut self) -> Result<(), Error> {
        self.protocol.session_end()
    }

    fn write_message(&mut self, message: super::ProtoMessage) -> Result<(), Error> {
        self.protocol.write(message)
    }

    fn read_message(&mut self) -> Result<super::ProtoMessage, Error> {
        self.protocol.read()
    }
}
