use super::token::Token;

#[derive(Debug)]
pub struct Program {
    stmts: Vec<Stmt>,
    errors: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Self { 
            stmts: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn stmts(&self) -> &Vec<Stmt> {
        &self.stmts
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn push_stmt(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }

    pub fn push_error(&mut self, error: String) {
        self.errors.push(error);
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
        self.stmts
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let(Expr, Expr),
    Ret(Expr),
    Expr(Expr),
}

impl ToString for Stmt {
    fn to_string(&self) -> String {
        match self {
            Self::Let(name, value) => {
                format!("let {} = {};", name.to_string(), value.to_string())
            }
            Self::Ret(value) => {
                format!("return {};", value.to_string())
            }
            Self::Expr(expr) => expr.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(String),
    Int(i64),
    Prefix(Token, Box<Expr>),
    Infix(Token, Box<Expr>, Box<Expr>),
    Value,
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Ident(name) => name.to_string(),
            Expr::Int(n) => n.to_string(),
            Expr::Prefix(op, expr) => {
                format!("({}{})", op.to_string(), expr.to_string())
            }
            Expr::Infix(op, l, r) => {
                format!("({}{}{})", l.to_string(), op.to_string(), r.to_string())
            },
            Expr::Value => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_works() {
        let program = Program {
            stmts: vec![Stmt::Let(
                Expr::Ident("a".to_string()),
                Expr::Ident("b".to_string()),
            )],
            errors: vec![],
        };

        assert_eq!(program.to_string(), "let a = b;".to_string());
    }
}