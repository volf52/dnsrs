// use anyhow::Result;
use color_eyre::eyre::Result;
use dnsrs::dns::{DNSAnswer, DNSBuffer, DNSHeader, DNSQuestion};
use tokio::{fs::File, io::AsyncReadExt, net::UdpSocket};

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
    println!("Bytes read: {}", bytes_read);

    let mut buff = DNSBuffer::from(query_bytes);
    let qheader = DNSHeader::try_from(&mut buff)?;
    let q = DNSQuestion::try_from(&mut buff)?;

    println!("{:?}", buff);
    println!("{}", qheader);
    println!("{}", q);

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
        let mut respb = DNSBuffer::from(&resp_buff[..n_received]);
        let resp_header = DNSHeader::try_from(&mut respb)?;
        let numq = resp_header.num_q() as usize;
        let numans = resp_header.num_ans() as usize;

        let mut questions = Vec::<DNSQuestion>::with_capacity(numq);
        let mut answers = Vec::<DNSAnswer>::with_capacity(numans);

        for _ in 0..numq {
            let q = DNSQuestion::try_from(&mut respb)?;
            questions.push(q);
        }

        for _ in 0..numans {
            let ans = DNSAnswer::try_from(&mut respb)?;
            answers.push(ans);
        }

        println!(" ------- Header -----------\n{}", resp_header);
        println!(" ------- Questions --------");
        for q in &questions {
            println!("{}\n", q);
        }

        println!(" ------- Answers --------");
        for ans in &answers {
            println!("{}\n", ans);
        }
    }

    // {
    //     let mut respf = File::create("lala").await?;
    //     let _ = respf.write(&resp_buff[..n_received]).await?;
    //
    //     respf.sync_all().await?;
    // }

    Ok(())
}
