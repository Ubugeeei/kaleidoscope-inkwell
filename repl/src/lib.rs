use std::io::Write;

use inkwell::{context::Context, passes::PassManager, OptimizationLevel};

use compile::Compiler;
use parse::Parser;

#[no_mangle]
pub extern "C" fn put_chard(x: f64) -> f64 {
    print!("{}", x as u8 as char);
    let _ = std::io::stdout().flush();
    x
}

#[no_mangle]
pub extern "C" fn print_d(x: f64) -> f64 {
    println!("{}", x);
    x
}

// Adding the functions above to a global array,
// so Rust compiler won't remove them.
#[used]
static EXTERNAL_FNS: [extern "C" fn(f64) -> f64; 2] = [put_chard, print_d];

/// Entry point of the program; acts as a REPL.
pub fn start() {
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();

    // Create FPM
    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let mut previous_exprs = Vec::new();

    loop {
        println!();
        print!("?> ");
        let _ = std::io::stdout().flush();

        // Read input from stdin
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read from standard input.");

        if input.starts_with("exit") || input.starts_with("quit") {
            break;
        } else if input.chars().all(char::is_whitespace) {
            continue;
        }

        // make module
        let module = context.create_module("tmp");

        // recompile every previously parsed function into the new module
        for prev in &previous_exprs {
            Compiler::compile(&context, &builder, &fpm, &module, prev)
                .expect("Cannot re-add previously compiled function.");
        }

        let (name, is_anonymous) = match Parser::new(input).parse() {
            Ok(fun) => {
                let is_anon = fun.is_anon;

                match Compiler::compile(&context, &builder, &fpm, &module, &fun) {
                    Ok(function) => {
                        if !is_anon {
                            // only add it now to ensure it is correct
                            previous_exprs.push(fun);
                        }

                        (function.get_name().to_str().unwrap().to_string(), is_anon)
                    }
                    Err(err) => {
                        println!("!> Error compiling function: {}", err);
                        continue;
                    }
                }
            }
            Err(err) => {
                println!("!> Error parsing expression: {}", err);
                continue;
            }
        };

        if is_anonymous {
            let ee = module
                .create_jit_execution_engine(OptimizationLevel::None)
                .unwrap();

            let maybe_fn =
                unsafe { ee.get_function::<unsafe extern "C" fn() -> f64>(name.as_str()) };
            let compiled_fn = match maybe_fn {
                Ok(f) => f,
                Err(err) => {
                    println!("!> Error during execution: {:?}", err);
                    continue;
                }
            };

            unsafe {
                println!("=> {}", compiled_fn.call());
            }
        }
    }
}
