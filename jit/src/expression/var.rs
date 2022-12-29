use ast::Expression;
use inkwell::values::FloatValue;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(super) fn compile_var(&mut self, name: &String) -> Result<FloatValue<'ctx>, &'static str> {
        match self.variables.get(name.as_str()) {
            Some(var) => Ok(self
                .builder
                .build_load(*var, name.as_str())
                .into_float_value()),
            None => Err("Could not find a matching variable."),
        }
    }

    pub(super) fn compile_var_in(
        &mut self,
        variables: &Vec<(String, Option<Expression>)>,
        body: &Box<Expression>,
    ) -> Result<FloatValue<'ctx>, &'static str> {
        let mut old_bindings = Vec::new();

        for &(ref var_name, ref initializer) in variables {
            let var_name = var_name.as_str();

            let initial_val = match *initializer {
                Some(ref init) => self.compile_expr(init)?,
                None => self.context.f64_type().const_float(0.),
            };

            let alloca = self.create_entry_block_alloca(var_name);

            self.builder.build_store(alloca, initial_val);

            if let Some(old_binding) = self.variables.remove(var_name) {
                old_bindings.push(old_binding);
            }

            self.variables.insert(var_name.to_string(), alloca);
        }

        let body = self.compile_expr(body)?;

        for binding in old_bindings {
            self.variables
                .insert(binding.get_name().to_str().unwrap().to_string(), binding);
        }

        Ok(body)
    }
}
