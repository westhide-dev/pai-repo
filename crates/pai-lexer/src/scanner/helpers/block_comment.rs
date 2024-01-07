use pai_error::{err, PResult};

use crate::scanner::{comment::Comment, unit::Unit, Scanner};

impl<'s> Scanner<'s> {
    pub fn scan_block_comment(&mut self) -> PResult<Unit<'s>> {
        self.mark();

        while !self.is_empty() {
            if self.byte() == b'*' {
                if self.peek(1) == b'/' {
                    self.down();

                    self.skip(2);

                    return Ok(unit!(BlockComment: self.raw()))
                } else {
                    self.skip(1)
                }
            }

            self.skip_char()
        }

        err!("Unterminated block comment")
    }
}
