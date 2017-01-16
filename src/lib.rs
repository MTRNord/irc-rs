extern crate bufstream;
mod irc {
    fn connect(host: String, port: u16) {
      use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
      use std::io::Write;
      use bufstream::BufStream;
      let host_split: Vec<&str> = host.split('.').collect();
      let ServerAddress = IpAddr::V4(Ipv4Addr::new(host_split[0].as_bytes()[0], host_split[1].as_bytes()[0], host_split[2].as_bytes()[0], host_split[3].as_bytes()[0])); // Connect to localhost
      let SocketAddress = SocketAddr::new(ServerAddress, port);
      let mut isConnected = false;

      let mut stream = BufStream::new(TcpStream::connect(SocketAddress).unwrap());

      stream.write(b"NICK rustbot\r\n");
      stream.write(b"USER rustbot 8 * :rustbot\r\n");
      stream.flush();
    }
}
