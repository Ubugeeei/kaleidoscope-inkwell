use ast::Expression;
use inkwell::values::FloatValue;

use crate::Compiler;

mod call;
mod for_;
mod if_;
mod ops;
mod var;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(super) fn compile_expr(
        &mut self,
        expr: &Expression,
    ) -> Result<FloatValue<'ctx>, &'static str> {
        match *expr {
            Expression::Number(nb) => Ok(self.context.f64_type().const_float(nb)),

            Expression::Variable(ref name) => self.compile_var(name),

            Expression::VarIn {
                ref variables,
                ref body,
            } => self.compile_var_in(variables, body),

            Expression::Binary {
                op,
                ref left,
                ref right,
            } => self.compile_binary_expr(op, left, right),

            Expression::Call {
                ref fn_name,
                ref args,
            } => self.compile_call_expression(fn_name, args),

            Expression::Conditional {
                ref cond,
                ref consequence,
                ref alternative,
            } => self.compile_if_expression(cond, consequence, alternative),

            Expression::For {
                ref var_name,
                ref start,
                ref end,
                ref step,
                ref body,
            } => self.compile_for_expression(var_name, start, end, step, body),
        }
    }
}
