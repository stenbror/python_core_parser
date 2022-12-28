pub use crate::location::Location;

#[derive(Clone, Debug, PartialEq)]
pub enum ExprContext {
    Load,
    Store,
    Del,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Boolop {
    And,
    Or,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Unaryop {
    Invert,
    Not,
    UAdd,
    USub,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Cmpop {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    BooleanOperator {
        location: Location,
        op: Boolop,
        values: Vec<Expression>
    },
    NamedExpression {
        location: Location,
        target: Box<Expression>,
        value: Box<Expression>
    },
    BinaryOperator {
        location: Location,
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>
    },
    UnaryOperator {
        location: Location,
        op: Unaryop,
        operand: Box<Expression>
    }

}