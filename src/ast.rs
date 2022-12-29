pub use crate::location::Location;

type Ident = String;

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
pub struct Arguments {

}

#[derive(Clone, Debug, PartialEq)]
pub struct Comprehension {
    pub target: Expression,
    pub iter: Expression,
    pub ifs: Vec<Expression>,
    pub is_async: usize,
}


// Python Base ASTNode ////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    pub location: Location,
    pub node: T
}

impl<T> Node<T> {
    pub fn new(location: Location, node: T) -> Self {
        Node {
            location,
            node
        }
    }
}


// Python Expression nodes ////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind {
    BooleanOperator {
        op: Boolop,
        values: Vec<Expression>
    },
    NamedExpression {
        target: Box<Expression>,
        value: Box<Expression>
    },
    BinaryOperator {
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>
    },
    UnaryOperator {
        op: Unaryop,
        operand: Box<Expression>
    },
    Lambda {
        args: Box<Arguments>,
        body: Box<Expression>
    },
    TestExpression {
        test: Box<Expression>,
        body: Box<Expression>,
        orelse: Box<Expression>
    },
    Dictionary {
        keys: Vec<Expression>,
        values: Vec<Expression>
    },
    Set {
        elements: Vec<Expression>
    },
    ListComp {
        element: Box<Expression>,
        generators: Vec<Comprehension>
    },
    SetComp {
        element: Box<Expression>,
        generators: Vec<Comprehension>
    },
    DictComp {
        keys: Vec<Expression>,
        values: Vec<Expression>,
        generators: Vec<Comprehension>
    },
    GeneratorExpr {
        element: Box<Expression>,
        generators: Vec<Comprehension>
    },
    Await {
        value: Box<Expression>
    },
    Yield {
        value: Option<Box<Expression>>
    },
    YieldFrom {
        value: Box<Expression>
    },
    Compare {
        left: Box<Expression>,
        ops: Vec<Cmpop>,
        comperators: Vec<Expression>
    },
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
        //keywords: Vec<Keyword>
    },
    FormattedValue {
        value: Box<Expression>,
        conversion: usize,
        format_spec: Option<Box<Expression>>
    },
    JoinedStr {
        values: Vec<Expression>
    },
    Constant {
        //value: Constant,
        kind: Option<String>
    },
    Attribute {
        value: Box<Expression>,
        attr: Ident,
        ctx: ExprContext
    },
    Subscript {
        value: Box<Expression>,
        slice: Box<Expression>,
        ctx: ExprContext
    },
    Starred {
        value: Box<Expression>,
        ctx: ExprContext
    },
    Name {
        id: Ident,
        ctx: ExprContext   
    },
    List {
        elements: Vec<Expression>,
        ctx: ExprContext
    },
    Tuple {
        elements: Vec<Expression>,
        ctx: ExprContext
    },
    Slice {
        lower: Option<Box<Expression>>,
        upper: Option<Box<Expression>>,
        step: Option<Box<Expression>>
    }
}

type Expression = Node<ExpressionKind>;