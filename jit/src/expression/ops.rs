use crate::Compiler;

use std::borrow::Borrow;

use ast::Expression;
use inkwell::{values::FloatValue, FloatPredicate};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(super) fn compile_binary_expr(
        &mut self,
        op: char,
        left: &Box<Expression>,
        right: &Box<Expression>,
    ) -> Result<FloatValue<'ctx>, &'static str> {
        if op == '=' {
            // handle assignment
            let var_name = match *left.borrow() {
                Expression::Variable(ref var_name) => var_name,
                _ => {
                    return Err("Expected variable as left-hand operator of assignment.");
                }
            };

            let var_val = self.compile_expr(right)?;
            let var = self
                .variables
                .get(var_name.as_str())
                .ok_or("Undefined variable.")?;

            self.builder.build_store(*var, var_val);

            Ok(var_val)
        } else {
            let lhs = self.compile_expr(left)?;
            let rhs = self.compile_expr(right)?;

            match op {
                '+' => Ok(self.builder.build_float_add(lhs, rhs, "tmp_add")),
                '-' => Ok(self.builder.build_float_sub(lhs, rhs, "tmp_sub")),
                '*' => Ok(self.builder.build_float_mul(lhs, rhs, "tmp_mul")),
                '/' => Ok(self.builder.build_float_div(lhs, rhs, "tmp_div")),
                '<' => Ok({
                    let cmp =
                        self.builder
                            .build_float_compare(FloatPredicate::ULT, lhs, rhs, "tmp_cmp");

                    self.builder.build_unsigned_int_to_float(
                        cmp,
                        self.context.f64_type(),
                        "tmp_bool",
                    )
                }),
                '>' => Ok({
                    let cmp =
                        self.builder
                            .build_float_compare(FloatPredicate::ULT, rhs, lhs, "tmp_cmp");

                    self.builder.build_unsigned_int_to_float(
                        cmp,
                        self.context.f64_type(),
                        "tmp_bool",
                    )
                }),

                custom => {
                    let mut name = String::from("binary");

                    name.push(custom);

                    match self.get_function(name.as_str()) {
                        Some(fun) => {
                            match self
                                .builder
                                .build_call(fun, &[lhs.into(), rhs.into()], "tmp_bin")
                                .try_as_basic_value()
                                .left()
                            {
                                Some(value) => Ok(value.into_float_value()),
                                None => Err("Invalid call produced."),
                            }
                        }

                        None => Err("Undefined binary operator."),
                    }
                }
            }
        }
    }
}
