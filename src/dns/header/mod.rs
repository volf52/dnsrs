mod flag;
mod mask;

use flag::HeaderFlags;

use super::buffer::DNSBuffer;
use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DNSHeader {
    id: u16,
    flags: HeaderFlags,
    n_questions: u16,
    n_answers: u16,
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

impl DNSHeader {
    pub fn from_buff(buff: &mut DNSBuffer) -> Result<Self> {
        let id = buff.read_u16()?;
        let flag_val = buff.read_u16()?;
        let n_questions = buff.read_u16()?;
        let n_answers = buff.read_u16()?;

        buff.read_u32()?; // Discard next 2 countes

        let flags: HeaderFlags = flag_val.into();

        Ok(Self {
            id,
            n_questions,
            n_answers,
            flags,
        })
    }
}
