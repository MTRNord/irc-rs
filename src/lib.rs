//! A crate for the irc chat  protocol
//!
//!
//! # Usage
//!
//! ```toml
//! [dependencies]
//! irc = "0.1"
//! ```
//!
//! ```no_run
//! use irc::client::*;
//!
//!
//! let host: String = format!("130.83.198.4");
//! let username: String = format!("nordbot");
//! let stream = connect(host, 6667, &username, &username);
//! ```

extern crate mio;

#[cfg(test)]
mod tests {
    #[test]
    fn Tconnect() {
        use client::connect;
        let host: String = format!("148.251.206.139");
        let username: String = format!("nordbot");
        let stream = connect(host, 6667, &username, &username);
    }
}

pub mod client {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::io::{Write, Read};
    use mio::tcp::TcpStream;
    /// Returns a TcpStream which is connected to the defined IRC Server
    ///
    /// # Arguments
    ///
    /// * `host` - A String which contains a IPv4 to the IRC Serevr
    /// * `port` - A u16 integer which contains the port of the IRC Server
    /// * `username` - A String containing the username with that the lib should connect to the IRC Server
    /// * `realname` - A String which contains the realname of the bot
    ///
    /// # Example
    ///
    /// ```
    /// use irc::client::*;
    ///
    /// let host: String = format!("130.83.198.4");
    /// let username: String = format!("nordbot");
    /// let stream = connect(host, 6667, &username, &username);
    ///
    /// ```
    pub fn connect(host: String, port: u16, username: &String, realname: &String) -> TcpStream {
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
        let mut isConnected = false;
        let mut stream = TcpStream::connect(&SocketAddress).unwrap();
        let nick = format!("NICK {}\r\n", username);
        let user = format!("USER {} 0 * : {}\r\n", username, realname);

        println!("{:?}", stream);

        stream.write(nick.as_bytes());
        stream.write(user.as_bytes());
        //stream.flush();

        let mut g = 1;
        while g > 0 {
            let mut r = String::new();
            stream.read_to_string(&mut r);
            let server_message_raw = r.clone();
            while server_message_raw.len() > 0 {
                println!("raw: {:?}", r);
                //println!("len: {:?}", r.len());
                if server_message_raw.is_empty(){
                    g = 0;
                }else{
                    if server_message_raw.contains("004")  {
                        isConnected = true;
                        println!("connected: {}", isConnected);
                    } else if server_message_raw.contains("PING") {
                        let mut server_message_vec: Vec<&str> = server_message_raw.split("\n").collect();
                        server_message_vec.pop();
                        let server_message_ping_index = server_message_vec.iter().position(|&r| r.contains("PING")).unwrap();
                        let server_message: Vec<&str> = server_message_vec[server_message_ping_index].split(" ").collect();
                        let pong = format!("PONG {}\r\n", server_message[1].trim_left());
                        stream.write(pong.as_bytes());
                        //stream.flush();
                        println!("{}", pong);
                    }
                    g = 1;
                }
            }
        }
        return stream;
    }
}
