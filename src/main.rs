// use anyhow::Result;
use color_eyre::eyre::Result;
use dnsrs::dns::buffer::Buffer;
// use dnsrs::errors::DNSError;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::UdpSocket,
};

const HOST: &str = "8.8.8.8";
const PORT: u16 = 53;
const DNS_SERVER: (&str, u16) = (HOST, PORT);
const ONE_KB: usize = 1024;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut query_buff = [0; ONE_KB];
    let bytes_read = {
        let mut file = File::open("test_data/query_packet").await?;
        file.read(&mut query_buff).await?
    };
    let query_bytes = &query_buff[..bytes_read];

    let buff = Buffer::from(query_bytes);
    println!("{:?}", buff);
    println!("{:?}", query_bytes);
    println!("Size: {}", std::mem::size_of::<Buffer>());

    println!("Bytes read: {}", bytes_read);

    let tmp = vec![240, 159, 146, 150];
    let mut b = Buffer::from(&tmp[..]);
    let mut b2 = Buffer::with_capacity(4);
    let s = b.read_string(4)?;
    println!("{}", s);
    b2.write_string(s);
    println!("{:?}", b2);

    let mut resp_buff = [0; ONE_KB];

    let n_received = {
        let sock = UdpSocket::bind("0.0.0.0:8000").await?;

        sock.connect(DNS_SERVER).await?;
        println!("Connected to DNS server");

        sock.writable().await?;
        let t = sock.send(query_bytes).await?;
        println!("Sent {} bytes", t);

        sock.readable().await?;
        let received = sock.recv(&mut resp_buff).await?;
        println!("Received {} bytes", received);

        received
    };

    {
        let mut respf = File::create("lala").await?;
        let _ = respf.write(&resp_buff[..n_received]).await?;

        respf.sync_all().await?;
    }

    Ok(())
}
