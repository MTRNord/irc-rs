extern crate bufstream;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Tconnect() {
        use irc::connect;
        let host: String = format!("130.83.198.4");
        let username: String = format!("testbot");
        let stream = connect(host, 6667, &username, &username);
    }
}

mod irc {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
    use std::io::{Write, BufRead};
    use bufstream::BufStream;
    pub fn connect(host: String, port: u16, username: &String, realname: &String) -> BufStream<TcpStream> {
        let host_split: Vec<&str> = host.split('.').collect();
        let ip1_1: i32 = host_split[0].parse().unwrap();
        let ip1_2: u8 = ip1_1 as _;
        let ip2_1: i32 = host_split[1].parse().unwrap();
        let ip2_2: u8 = ip2_1 as _;
        let ip3_1: i32 = host_split[2].parse().unwrap();
        let ip3_2: u8 = ip3_1 as _;
        let ip4_1: i32 = host_split[3].parse().unwrap();
        let ip4_2: u8 = ip4_1 as _;
        let ServerAddress = IpAddr::V4(Ipv4Addr::new(ip1_2, ip2_2, ip3_2, ip4_2)); // Connect to localhost
        let SocketAddress = SocketAddr::new(ServerAddress, port);

        println!("split: {}", host_split[0]);
        println!("int bytes: {}", ip1_2);
        println!("bytes: {}", host_split[0].as_bytes()[1]);
        println!("bytes: {}", SocketAddress);
        let mut isConnected = false;
        let mut stream = BufStream::new(TcpStream::connect(SocketAddress).unwrap());
        let nick = format!("NICK {}\r\n", username);
        let user = format!("USER {} 0 * : {}\r\n", username, realname);

        stream.write(nick.as_bytes());
        stream.write(user.as_bytes());
        stream.flush();

        let mut g = 1;
        while g > 0 {
            let mut r = String::new();

            while stream.read_line(&mut r).unwrap() > 0 {
                if r.is_empty(){
                    g = 0;
                }else{
                    if r.clone().contains("004")  {
                        isConnected = true;
                    } else if r.clone().contains("PING") {
                        stream.write(b"PONG\r\n");
                        stream.flush();
                    }
                }
            }
        }
        return stream;
    }
}
