use std::net::SocketAddr;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34253")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let buf: [u8; 4] = [1, 2, 3, 4];
        socket.send_to(&buf, SocketAddr::from(([127, 0, 0, 1], 34254)))?;
    } // the socket is closed here
    Ok(())
}
