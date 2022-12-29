use ast::Expression;
use inkwell::{values::FloatValue, FloatPredicate};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(super) fn compile_if_expression(
        &mut self,
        cond: &Box<Expression>,
        consequence: &Box<Expression>,
        alternative: &Box<Expression>,
    ) -> Result<FloatValue<'ctx>, &'static str> {
        let parent = self.fn_value();
        let zero_const = self.context.f64_type().const_float(0.0);

        // create condition by comparing without 0.0 and returning an int
        let cond = self.compile_expr(cond)?;
        let cond =
            self.builder
                .build_float_compare(FloatPredicate::ONE, cond, zero_const, "if_cond");

        // build branch
        let then_bb = self.context.append_basic_block(parent, "then");
        let else_bb = self.context.append_basic_block(parent, "else");
        let cont_bb = self.context.append_basic_block(parent, "if_cont");

        self.builder
            .build_conditional_branch(cond, then_bb, else_bb);

        // build then block
        self.builder.position_at_end(then_bb);
        let then_val = self.compile_expr(consequence)?;
        self.builder.build_unconditional_branch(cont_bb);

        let then_bb = self.builder.get_insert_block().unwrap();

        // build else block
        self.builder.position_at_end(else_bb);
        let else_val = self.compile_expr(alternative)?;
        self.builder.build_unconditional_branch(cont_bb);

        let else_bb = self.builder.get_insert_block().unwrap();

        // emit merge block
        self.builder.position_at_end(cont_bb);

        let phi = self.builder.build_phi(self.context.f64_type(), "if_tmp");

        phi.add_incoming(&[(&then_val, then_bb), (&else_val, else_bb)]);

        Ok(phi.as_basic_value().into_float_value())
    }
}
