use crate::DNSError;

use super::{DNSAnswer, DNSBuffer, DNSHeader, DNSQuestion};

#[derive(Debug)]
pub struct DNSPacket {
    header: DNSHeader,
    questions: Vec<DNSQuestion>,
    answers: Vec<DNSAnswer>,
}

impl TryFrom<&[u8]> for DNSPacket {
    type Error = DNSError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut buff = DNSBuffer::from(value);

        Self::try_from(&mut buff)
    }
}

impl TryFrom<&mut DNSBuffer> for DNSPacket {
    type Error = DNSError;

    fn try_from(buff: &mut DNSBuffer) -> Result<Self, Self::Error> {
        let header = DNSHeader::try_from(&mut *buff)?;

        let n_q = header.num_q() as usize;
        let n_ans = header.num_ans() as usize;

        let mut questions = Vec::<DNSQuestion>::with_capacity(n_q);
        let mut answers = Vec::<DNSAnswer>::with_capacity(n_ans);

        for _ in 0..n_q {
            let q = DNSQuestion::try_from(&mut *buff)?;
            questions.push(q);
        }

        for _ in 0..n_ans {
            let ans = DNSAnswer::try_from(&mut *buff)?;

            answers.push(ans);
        }

        Ok(Self {
            header,
            questions,
            answers,
        })
    }
}

impl std::fmt::Display for DNSPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----------- Header -----------\n{}", self.header)?;
        writeln!(f, "----------- Questions -----------")?;
        for q in &self.questions {
            writeln!(f, "{}", q)?;
        }
        writeln!(f, "----------- Answers -----------")?;
        for ans in &self.answers {
            writeln!(f, "{}", ans)?;
        }

        Ok(())
    }
}
