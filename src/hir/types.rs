//! HIR Type Definitions
//!
//! The 22 canonical HIR node types.

use crate::ast::Span;

/// Top-level HIR node
#[derive(Debug, Clone, PartialEq)]
pub enum HirNode {
    Type(HirType),
    Expr(HirExpr),
    Stmt(HirStmt),
}

/// Type definitions
#[derive(Debug, Clone, PartialEq)]
pub enum HirType {
    /// gene → struct
    Struct {
        name: String,
        fields: Vec<HirField>,
        span: Span,
    },
    /// enum variants
    Enum {
        name: String,
        variants: Vec<HirVariant>,
        span: Span,
    },
    /// trait → interface
    Interface {
        name: String,
        methods: Vec<HirMethod>,
        span: Span,
    },
}

/// Expression nodes
#[derive(Debug, Clone, PartialEq)]
pub enum HirExpr {
    Literal {
        value: HirLiteral,
        span: Span,
    },
    Ident {
        name: String,
        span: Span,
    },
    Binary {
        op: HirBinOp,
        left: Box<HirExpr>,
        right: Box<HirExpr>,
        span: Span,
    },
    Unary {
        op: HirUnaryOp,
        operand: Box<HirExpr>,
        span: Span,
    },
    Call {
        callee: Box<HirExpr>,
        args: Vec<HirExpr>,
        span: Span,
    },
    Field {
        object: Box<HirExpr>,
        field: String,
        span: Span,
    },
    Index {
        object: Box<HirExpr>,
        index: Box<HirExpr>,
        span: Span,
    },
    Lambda {
        params: Vec<HirParam>,
        body: Box<HirExpr>,
        span: Span,
    },
    If {
        condition: Box<HirExpr>,
        then_branch: Box<HirExpr>,
        else_branch: Option<Box<HirExpr>>,
        span: Span,
    },
    Match {
        scrutinee: Box<HirExpr>,
        arms: Vec<HirMatchArm>,
        span: Span,
    },
    Block {
        stmts: Vec<HirStmt>,
        expr: Option<Box<HirExpr>>,
        span: Span,
    },
}

/// Statement nodes
#[derive(Debug, Clone, PartialEq)]
pub enum HirStmt {
    Binding {
        name: String,
        mutable: bool,
        ty: Option<HirTypeRef>,
        value: HirExpr,
        span: Span,
    },
    Assign {
        target: HirExpr,
        value: HirExpr,
        span: Span,
    },
    Return {
        value: Option<HirExpr>,
        span: Span,
    },
    Loop {
        kind: HirLoopKind,
        body: Vec<HirStmt>,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Expr {
        expr: HirExpr,
        span: Span,
    },
}

// Supporting types
#[derive(Debug, Clone, PartialEq)]
pub struct HirField {
    pub name: String,
    pub ty: HirTypeRef,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirVariant {
    pub name: String,
    pub fields: Vec<HirField>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirMethod {
    pub name: String,
    pub params: Vec<HirParam>,
    pub return_ty: HirTypeRef,
    pub body: Option<HirExpr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirParam {
    pub name: String,
    pub ty: HirTypeRef,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirMatchArm {
    pub pattern: HirPattern,
    pub guard: Option<HirExpr>,
    pub body: HirExpr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirPattern {
    Wildcard {
        span: Span,
    },
    Ident {
        name: String,
        span: Span,
    },
    Literal {
        value: HirLiteral,
        span: Span,
    },
    Variant {
        name: String,
        fields: Vec<HirPattern>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirLiteral {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Char(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirUnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirLoopKind {
    Loop,
    While { condition: HirExpr },
    ForIn { var: String, iter: HirExpr },
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirTypeRef {
    pub name: String,
    pub params: Vec<HirTypeRef>,
    pub span: Span,
}

impl std::fmt::Display for HirBinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HirBinOp::Add => write!(f, "+"),
            HirBinOp::Sub => write!(f, "-"),
            HirBinOp::Mul => write!(f, "*"),
            HirBinOp::Div => write!(f, "/"),
            HirBinOp::Mod => write!(f, "%"),
            HirBinOp::Eq => write!(f, "=="),
            HirBinOp::Ne => write!(f, "!="),
            HirBinOp::Lt => write!(f, "<"),
            HirBinOp::Le => write!(f, "<="),
            HirBinOp::Gt => write!(f, ">"),
            HirBinOp::Ge => write!(f, ">="),
            HirBinOp::And => write!(f, "&&"),
            HirBinOp::Or => write!(f, "||"),
        }
    }
}
