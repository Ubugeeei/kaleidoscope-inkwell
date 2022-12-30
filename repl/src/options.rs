#[derive(Debug, Clone, Copy)]
pub struct ReplOptions {
    pub(super) emit_ir: bool,
    // pub(super) emit_ast: bool,
    // pub(super) emit_byte_code: bool,
    // pub(super) emit_assembly: bool,
}

impl Default for ReplOptions {
    fn default() -> Self {
        ReplOptions {
            emit_ir: false,
            // emit_ast: false,
            // emit_byte_code: false,
            // emit_assembly: false,
        }
    }
}

impl ReplOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn emit_ir(&mut self, emit_ir: bool) -> Self {
        self.emit_ir = emit_ir;
        *self
    }

    // pub fn emit_ast(&mut self, emit_ast: bool) -> Self {
    //     self.emit_ast = emit_ast;
    //     *self
    // }

    // pub fn emit_byte_code(&mut self, emit_byte_code: bool) -> Self {
    //     self.emit_byte_code = emit_byte_code;
    //     *self
    // }

    // pub fn emit_assembly(&mut self, emit_assembly: bool) -> Self {
    //     self.emit_assembly = emit_assembly;
    //     *self
    // }
}
