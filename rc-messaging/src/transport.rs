const BUF_SIZE: usize = 1024;
const MESSAGE_TIMEOUT_HZ: f64 = 20.0;

const MESSAGE_TIMEOUT: std::time::Duration =
    std::time::Duration::from_millis((1.0 / MESSAGE_TIMEOUT_HZ * 1000.0) as u64);
const READ_TIMEOUT: std::time::Duration = MESSAGE_TIMEOUT;
const WRITE_TIMEOUT: std::time::Duration = MESSAGE_TIMEOUT;

fn get_socket(bind_address: std::net::SocketAddr) -> Result<std::net::UdpSocket, std::io::Error> {
    let socket = std::net::UdpSocket::bind(bind_address)?;
    socket.set_read_timeout(Some(READ_TIMEOUT))?;
    socket.set_write_timeout(Some(WRITE_TIMEOUT))?;

    Ok(socket)
}

fn get_input_message_sender_and_receiver() -> (
    std::sync::mpsc::Sender<crate::serialization::InputMessage>,
    std::sync::mpsc::Receiver<crate::serialization::InputMessage>,
) {
    std::sync::mpsc::channel()
}

pub struct Server {
    bind_address: std::net::SocketAddr,
    socket: std::net::UdpSocket,
    incoming_input_message_sender: std::sync::mpsc::Sender<crate::serialization::InputMessage>,
    closed: std::sync::Arc<std::sync::Mutex<bool>>,
}

impl Server {
    pub fn new(
        bind_address: std::net::SocketAddr,
        incoming_input_message_sender: std::sync::mpsc::Sender<crate::serialization::InputMessage>,
    ) -> Result<Self, anyhow::Error> {
        let socket = get_socket(bind_address)?;

        Ok(Self {
            bind_address: socket.local_addr()?,
            socket,
            incoming_input_message_sender,
            closed: std::sync::Arc::new(std::sync::Mutex::new(false)),
        })
    }

    pub fn get_closer(&self) -> impl Fn() {
        let closed = std::sync::Arc::clone(&self.closed);
        return move || {
            let mut closed = closed.lock().unwrap();
            *closed = true;
        };
    }

    pub fn get_bind_address(&self) -> std::net::SocketAddr {
        self.bind_address
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let mut buf = vec![0; BUF_SIZE];

        loop {
            {
                let closed = self.closed.lock().unwrap();
                if *closed {
                    break;
                }
            }

            let recv_from_result = self.socket.recv_from(&mut buf);
            if recv_from_result.is_err() {
                let err = recv_from_result.err().unwrap();
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    continue;
                }

                println!("recv_from() failed because err={:?}", err);
                return Err(err.into());
            }

            let (n, _) = recv_from_result?;
            let buf = buf[0..n].to_vec();

            let input_message = crate::serialization::deserialize(buf)?;

            self.incoming_input_message_sender.send(input_message)?;
        }

        drop(self.incoming_input_message_sender.clone());

        Ok(())
    }
}

pub struct Client {
    send_address: std::net::SocketAddr,
    bind_address: std::net::SocketAddr,
    socket: std::net::UdpSocket,
    outgoing_input_message_sender: std::sync::mpsc::Sender<crate::serialization::InputMessage>,
    outgoing_input_message_receiver: std::sync::mpsc::Receiver<crate::serialization::InputMessage>,
    closed: std::sync::Arc<std::sync::Mutex<bool>>,
}

impl Client {
    pub fn new(send_address: std::net::SocketAddr) -> Result<Self, anyhow::Error> {
        let socket = get_socket("0.0.0.0:0".parse()?)?;

        let (outgoing_input_message_sender, outgoing_input_message_receiver) =
            get_input_message_sender_and_receiver();

        Ok(Self {
            send_address,
            bind_address: socket.local_addr()?,
            socket,
            outgoing_input_message_sender,
            outgoing_input_message_receiver,
            closed: std::sync::Arc::new(std::sync::Mutex::new(false)),
        })
    }

    pub fn get_closer(&self) -> impl Fn() {
        let closed = std::sync::Arc::clone(&self.closed);
        return move || {
            let mut closed = closed.lock().unwrap();
            *closed = true;
        };
    }

    pub fn get_bind_address(&self) -> std::net::SocketAddr {
        self.bind_address
    }

    pub fn get_send_address(&self) -> std::net::SocketAddr {
        self.send_address
    }

    pub fn get_outgoing_input_message_sender(
        &self,
    ) -> std::sync::mpsc::Sender<crate::serialization::InputMessage> {
        self.outgoing_input_message_sender.clone()
    }

    pub fn run(&self) -> anyhow::Result<()> {
        loop {
            {
                let closed = self.closed.lock().unwrap();
                if *closed {
                    break;
                }
            }

            let recv_timeout_result = self
                .outgoing_input_message_receiver
                .recv_timeout(std::time::Duration::from_secs(1));
            if recv_timeout_result.is_err() {
                continue;
            }

            let input_message = recv_timeout_result?;

            let buf = crate::serialization::serialize(&input_message)?;

            let _ = self.socket.send_to(&buf, self.send_address)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() -> anyhow::Result<()> {
        let (incoming_input_message_sender, incoming_input_message_receiver) =
            get_input_message_sender_and_receiver();

        let server = Server::new("0.0.0.0:0".parse()?, incoming_input_message_sender)?;
        let client = Client::new(server.get_bind_address())?;

        let server_closer = server.get_closer();
        let client_closer = client.get_closer();

        let outgoing_input_message_sender = client.get_outgoing_input_message_sender();

        let server_handle = std::thread::spawn(move || {
            server.run().unwrap();
        });

        let client_handle = std::thread::spawn(move || {
            client.run().unwrap();
        });

        let outgoing_input_message = crate::serialization::InputMessage {
            throttle: 0.69,
            steering: 0.69,
            throttle_left: 0.69,
            throttle_right: 0.69,
            steering_left: 0.69,
            steering_right: 0.69,
            mode_up: true,
            mode_down: true,
            mode_left: true,
            mode_right: true,
            handbrake: true,
        };

        let incoming_input_message = {
            outgoing_input_message_sender.send(outgoing_input_message.clone())?;
            incoming_input_message_receiver.recv()?
        };

        assert_eq!(outgoing_input_message, incoming_input_message);

        server_closer();
        client_closer();

        server_handle.join().unwrap();
        client_handle.join().unwrap();

        Ok(())
    }

    #[test]
    fn no_server() -> anyhow::Result<()> {
        let client = Client::new("127.0.0.1:13337".parse()?)?;

        let client_closer = client.get_closer();

        let outgoing_input_message_sender = client.get_outgoing_input_message_sender();

        let client_handle = std::thread::spawn(move || {
            client.run().unwrap();
        });

        let outgoing_input_message = crate::serialization::InputMessage {
            throttle: 0.69,
            steering: 0.69,
            throttle_left: 0.69,
            throttle_right: 0.69,
            steering_left: 0.69,
            steering_right: 0.69,
            mode_up: true,
            mode_down: true,
            mode_left: true,
            mode_right: true,
            handbrake: true,
        };

        outgoing_input_message_sender.send(outgoing_input_message.clone())?;

        client_closer();

        client_handle.join().unwrap();

        Ok(())
    }

    #[test]
    fn no_client() -> anyhow::Result<()> {
        let (incoming_input_message_sender, incoming_input_message_receiver) =
            get_input_message_sender_and_receiver();

        let server = Server::new("0.0.0.0:0".parse()?, incoming_input_message_sender)?;

        let server_closer = server.get_closer();

        let server_handle = std::thread::spawn(move || {
            server.run().unwrap();
        });

        _ = incoming_input_message_receiver.recv_timeout(std::time::Duration::from_secs(1));

        server_closer();

        server_handle.join().unwrap();

        Ok(())
    }
}
