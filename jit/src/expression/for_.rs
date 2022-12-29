use ast::Expression;
use inkwell::{values::FloatValue, FloatPredicate};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(super) fn compile_for_expression(
        &mut self,
        var_name: &String,
        start: &Box<Expression>,
        end: &Box<Expression>,
        step: &Option<Box<Expression>>,
        body: &Box<Expression>,
    ) -> Result<FloatValue<'ctx>, &'static str> {
        let parent = self.fn_value();

        let start_alloca = self.create_entry_block_alloca(var_name);
        let start = self.compile_expr(start)?;

        self.builder.build_store(start_alloca, start);

        // go from current block to loop block
        let loop_bb = self.context.append_basic_block(parent, "loop");

        self.builder.build_unconditional_branch(loop_bb);
        self.builder.position_at_end(loop_bb);

        let old_val = self.variables.remove(var_name.as_str());

        self.variables.insert(var_name.to_owned(), start_alloca);

        // emit body
        self.compile_expr(body)?;

        // emit step
        let step = match *step {
            Some(ref step) => self.compile_expr(step)?,
            None => self.context.f64_type().const_float(1.0),
        };

        // compile end condition
        let end_cond = self.compile_expr(end)?;

        let current_var = self.builder.build_load(start_alloca, var_name);
        let next_var =
            self.builder
                .build_float_add(current_var.into_float_value(), step, "next_var");

        self.builder.build_store(start_alloca, next_var);

        let end_cond = self.builder.build_float_compare(
            FloatPredicate::ONE,
            end_cond,
            self.context.f64_type().const_float(0.0),
            "loop_cond",
        );
        let after_bb = self.context.append_basic_block(parent, "after_loop");

        self.builder
            .build_conditional_branch(end_cond, loop_bb, after_bb);
        self.builder.position_at_end(after_bb);

        self.variables.remove(var_name);

        if let Some(val) = old_val {
            self.variables.insert(var_name.to_owned(), val);
        }

        Ok(self.context.f64_type().const_float(0.0))
    }
}
