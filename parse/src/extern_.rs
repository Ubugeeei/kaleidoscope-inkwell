use ast::Function;

use crate::Parser;

impl Parser {
    pub(super) fn parse_extern(&mut self) -> Result<Function, &'static str> {
        self.next();

        let proto = self.parse_prototype()?;

        Ok(Function {
            prototype: proto,
            body: None,
            is_anon: false,
        })
    }
}
