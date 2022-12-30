use repl::{options::ReplOptions, start};

fn main() {
    let emit_ir = std::env::args().any(|arg| arg == "--emit-ir");
    start(ReplOptions::new().emit_ir(emit_ir));
}
