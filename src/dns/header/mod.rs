mod flag;
mod mask;

use flag::HeaderFlags;

use super::buffer::DNSBuffer;
use crate::DNSError;

#[derive(Debug)]
pub struct DNSHeader {
    id: u16,
    flags: HeaderFlags,
    n_questions: u16,
    n_answers: u16,
}

impl DNSHeader {
    #[must_use]
    pub const fn num_q(&self) -> u16 {
        self.n_questions
    }

    #[must_use]
    pub const fn num_ans(&self) -> u16 {
        self.n_answers
    }
}

impl std::fmt::Display for DNSHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID = {}", self.id)?;
        self.flags.fmt(f)?;

        write!(
            f,
            "\nQuestions: {}\tAnswers: {}",
            self.n_questions, self.n_answers
        )
    }
}

impl TryFrom<&mut DNSBuffer> for DNSHeader {
    type Error = DNSError;

    fn try_from(buff: &mut DNSBuffer) -> std::result::Result<Self, Self::Error> {
        let id = buff.read_u16()?;
        let flag_val = buff.read_u16()?;
        let n_questions = buff.read_u16()?;
        let n_answers = buff.read_u16()?;

        buff.read_u32()?; // Discard next 2 counts

        let flags: HeaderFlags = flag_val.into();

        Ok(Self {
            id,
            flags,
            n_questions,
            n_answers,
        })
    }
}
