use ast::Expression;
use inkwell::values::{BasicMetadataValueEnum, FloatValue};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(super) fn compile_call_expression(
        &mut self,
        name: &str,
        args: &Vec<Expression>,
    ) -> Result<FloatValue<'ctx>, &'static str> {
        match self.get_function(name) {
            Some(fun) => {
                let mut compiled_args = Vec::with_capacity(args.len());

                for arg in args {
                    compiled_args.push(self.compile_expr(arg)?);
                }

                let argsv: Vec<BasicMetadataValueEnum> = compiled_args
                    .iter()
                    .by_ref()
                    .map(|&val| val.into())
                    .collect();

                match self
                    .builder
                    .build_call(fun, argsv.as_slice(), "tmp")
                    .try_as_basic_value()
                    .left()
                {
                    Some(value) => Ok(value.into_float_value()),
                    None => Err("Invalid call produced."),
                }
            }
            None => Err("Unknown function."),
        }
    }
}
