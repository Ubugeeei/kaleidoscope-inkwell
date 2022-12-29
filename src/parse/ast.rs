#[derive(Debug, PartialEq)]
pub enum Expression {
    Binary {
        op: char,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    Call {
        fn_name: String,
        args: Vec<Expression>,
    },

    Conditional {
        cond: Box<Expression>,
        consequence: Box<Expression>,
        alternative: Box<Expression>,
    },

    For {
        var_name: String,
        start: Box<Expression>,
        end: Box<Expression>,
        step: Option<Box<Expression>>,
        body: Box<Expression>,
    },

    VarIn {
        variables: Vec<(String, Option<Expression>)>,
        body: Box<Expression>,
    },

    Number(f64),

    Variable(String),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Option<Expression>,
    pub is_anon: bool,
}

#[derive(Debug, PartialEq)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>,
    pub is_op: bool,
    pub prec: usize,
}
