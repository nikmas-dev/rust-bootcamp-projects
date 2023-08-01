use std::borrow::Cow;
use std::net::{IpAddr, SocketAddr};

fn main() {
    println!("Refactor me!");

    let mut err = Error::new("NO_USER");
    err.status(404).message("User not found");
}

#[derive(Debug)]
pub struct Error<'a> {
    code: Cow<'a, str>,
    status: u16,
    message: Cow<'a, str>,
}

impl<'a> Default for Error<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            code: Cow::Borrowed("UNKNOWN"),
            status: 500,
            message: Cow::Borrowed("Unknown error has happened."),
        }
    }
}

impl<'a> Error<'a> {
    pub fn new(code: impl Into<Cow<'a, str>>) -> Self {
        Self {
            code: code.into(),
            ..Self::default()
        }
    }

    pub fn status(&mut self, s: u16) -> &mut Self {
        self.status = s;
        self
    }

    pub fn message(&mut self, m: impl Into<Cow<'a, str>>) -> &mut Self {
        self.message = m.into();
        self
    }
}

#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>);

impl Server {
    pub fn bind(&mut self, ip: IpAddr, port: u16) {
        self.0 = Some(SocketAddr::new(ip, port))
    }
}

#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use std::net::Ipv4Addr;

        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            server.bind(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            server.bind("::1".parse().unwrap(), 9911);
            assert_eq!(format!("{}", server.0.unwrap()), "[::1]:9911");
        }
    }
}
