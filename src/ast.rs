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

// Python Module nodes ////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleKind {
    Module {
        body: Vec<Statement>,
        type_ignores: Vec<TypeIgnore>,
    },
    Interactive {
        body: Vec<Statement>,
    },
    Expression {
        body: Box<Expression>,
    },
    FunctionType {
        argtypes: Vec<Expression>,
        returns: Box<Expression>,
    }
}

pub type Block = Node<ModuleKind>;

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

// Python Statement nodes /////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub enum StatementKind {
    FunctionDef {
        name: Ident,
        args: Box<Arguments>,
        body: Vec<Statement>,
        decorator_list: Vec<Expression>,
        returns: Option<Box<Expression>>,
        type_comment: Option<String>,
    },
    AsyncFunctionDef {
        name: Ident,
        args: Box<Arguments>,
        body: Vec<Statement>,
        decorator_list: Vec<Expression>,
        returns: Option<Box<Expression>>,
        type_comment: Option<String>,
    },
    ClassDef {
        name: Ident,
        bases: Vec<Expression>,
        keywords: Vec<Keyword>,
        body: Vec<Statement>,
        decorator_list: Vec<Expression>,
    },
    Return {
        value: Option<Box<Expression>>,
    },
    Delete {
        targets: Vec<Expression>,
    },
    Assign {
        targets: Vec<Expression>,
        value: Box<Expression>,
        type_comment: Option<String>,
    },
    AugAssign {
        target: Box<Expression>,
        op: Operator,
        value: Box<Expression>,
    },
    AnnAssign {
        target: Box<Expression>,
        annotation: Box<Expression>,
        value: Option<Box<Expression>>,
        simple: usize,
    },
    For {
        target: Box<Expression>,
        iter: Box<Expression>,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
        type_comment: Option<String>,
    },
    AsyncFor {
        target: Box<Expression>,
        iter: Box<Expression>,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
        type_comment: Option<String>,
    },
    While {
        test: Box<Expression>,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
    },
    If {
        test: Box<Expression>,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
    },
    With {
        items: Vec<Withitem>,
        body: Vec<Statement>,
        type_comment: Option<String>,
    },
    AsyncWith {
        items: Vec<Withitem>,
        body: Vec<Statement>,
        type_comment: Option<String>,
    },
    Match {
        subject: Box<Expression>,
        cases: Vec<MatchCase>,
    },
    Raise {
        exc: Option<Box<Expression>>,
        cause: Option<Box<Expression>>,
    },
    Try {
        body: Vec<Statement>,
        handlers: Vec<Excepthandler>,
        orelse: Vec<Statement>,
        finalbody: Vec<Statement>,
    },
    Assert {
        test: Box<Expression>,
        msg: Option<Box<Expression>>,
    },
    Import {
        names: Vec<Alias>,
    },
    ImportFrom {
        module: Option<Ident>,
        names: Vec<Alias>,
        level: Option<usize>,
    },
    Global {
        names: Vec<Ident>,
    },
    Nonlocal {
        names: Vec<Ident>,
    },
    Expr {
        value: Box<Expression>,
    },
    Pass,
    Break,
    Continue
}

pub type Statement = Node<StatementKind>;

// Python match node //////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub enum PatternKind {
    MatchValue {
        value: Box<Expression>,
    },
    MatchSingleton {
        value: Constant,
    },
    MatchSequence {
        patterns: Vec<Pattern>,
    },
    MatchMapping {
        keys: Vec<Expression>,
        patterns: Vec<Pattern>,
        rest: Option<Ident>,
    },
    MatchClass {
        cls: Box<Expression>,
        patterns: Vec<Pattern>,
        kwd_attrs: Vec<Ident>,
        kwd_patterns: Vec<Pattern>,
    },
    MatchStar {
        name: Option<Ident>,
    },
    MatchAs {
        pattern: Option<Box<Pattern>>,
        name: Option<Ident>,
    },
    MatchOr {
        patterns: Vec<Pattern>,
    },
}

pub type Pattern = Node<PatternKind>;

// Extra utillities ///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub enum ExcepthandlerKind {
    ExceptHandler {
        type_: Option<Box<Expression>>,
        name: Option<Ident>,
        body: Vec<Statement>,
    },
}
pub type Excepthandler = Node<ExcepthandlerKind>;

#[derive(Clone, Debug, PartialEq)]
pub struct Arguments {
    pub posonlyargs: Vec<Arg>,
    pub args: Vec<Arg>,
    pub vararg: Option<Box<Arg>>,
    pub kwonlyargs: Vec<Arg>,
    pub kw_defaults: Vec<Expression>,
    pub kwarg: Option<Box<Arg>>,
    pub defaults: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArgData {
    pub arg: Ident,
    pub annotation: Option<Box<Expression>>,
    pub type_comment: Option<String>
}
pub type Arg = Node<ArgData>;

#[derive(Clone, Debug, PartialEq)]
pub struct KeywordData {
    pub arg: Option<Ident>,
    pub value: Expression
}
pub type Keyword = Node<KeywordData>;

#[derive(Clone, Debug, PartialEq)]
pub struct AliasData {
    pub name: Ident,
    pub asname: Option<Ident>,
}
pub type Alias = Node<AliasData>;

#[derive(Clone, Debug, PartialEq)]
pub struct Withitem {
    pub context_expr: Expression,
    pub optional_vars: Option<Box<Expression>>
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub guard: Option<Box<Expression>>,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeIgnore {
    TypeIgnore { pos: usize, tag: String }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    None,
    Bool(bool),
    Str(String),
    Bytes(Vec<u8>),
    Int(i64),
    Tuple(Vec<Constant>),
    Float(f64),
    Complex { real: f64, imag: f64 },
    Ellipsis,
}