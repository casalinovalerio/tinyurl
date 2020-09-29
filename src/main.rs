use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use async_tls::TlsConnector;
use std::net::ToSocketAddrs;
use structopt::StructOpt;

static DOMAIN: &'static str = "tinyurl.com";
static ENDPOINT: &'static str = "/api-create.php";

#[derive(StructOpt)]
struct Options {
    url: String,
}

fn main() -> io::Result<()> {
    let options = Options::from_args();

    // Check if the provided host exists
    let addr = (DOMAIN, 443 as u16)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    let request = format!("GET {}?url={} HTTP/1.0\r\nHost: {}\r\n\r\n", 
                          ENDPOINT, 
                          options.url.as_str(), 
                          DOMAIN);

    task::block_on(async move {
        let connector = TlsConnector::default();
        let tcp_stream = TcpStream::connect(&addr).await?;
        let mut tls_stream = connector
            .connect(format!("{}",DOMAIN), tcp_stream)
            .await?;
        let mut stdout = io::stdout();

        tls_stream.write_all(request.as_bytes()).await?;
        io::copy(&mut tls_stream, &mut stdout).await?;
        Ok(())
    })
}
