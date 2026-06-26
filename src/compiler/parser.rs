use crate::compiler::lexer::Token;
use crate::errors::{Error, Result};

#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Vec<Statement>),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration {
        kind: VarKind,
        declarations: Vec<VariableDeclarator>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
        is_async: bool,
    },
    ReturnStatement(Option<Expression>),
    IfStatement {
        condition: Expression,
        consequent: Box<Statement>,
        alternate: Option<Box<Statement>>,
    },
    WhileStatement {
        condition: Expression,
        body: Box<Statement>,
    },
    BlockStatement(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub enum VarKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, Clone)]
pub struct VariableDeclarator {
    pub id: String,
    pub init: Option<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NullLiteral,
    UndefinedLiteral,
    Identifier(String),
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    Assignment {
        target: Box<Expression>,
        value: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
    Member {
        object: Box<Expression>,
        property: Box<Expression>,
        computed: bool,
    },
    FunctionExpression {
        name: Option<String>,
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    StrictEq,
    NotEqual,
    StrictNotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
    Typeof,
    Void,
    Delete,
}

pub fn parse(tokens: &[Token]) -> Result<AstNode> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }
    
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }
    
    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }
    
    fn expect(&mut self, expected: &Token) -> Result<()> {
        let token = self.advance();
        if token == expected {
            Ok(())
        } else {
            Err(Error::ParseError(format!("Expected {:?}, got {:?}", expected, token)))
        }
    }
    
    fn parse_program(&mut self) -> Result<AstNode> {
        let mut statements = Vec::new();
        
        while self.peek() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }
        
        Ok(AstNode::Program(statements))
    }
    
    fn parse_statement(&mut self) -> Result<Statement> {
        match self.peek() {
            Token::Const | Token::Let | Token::Var => self.parse_variable_declaration(),
            Token::Function => self.parse_function_declaration(),
            Token::Return => self.parse_return_statement(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::LeftBrace => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    
    fn parse_variable_declaration(&mut self) -> Result<Statement> {
        let kind = match self.advance() {
            Token::Var => VarKind::Var,
            Token::Let => VarKind::Let,
            Token::Const => VarKind::Const,
            _ => unreachable!(),
        };
        
        let mut declarations = Vec::new();
        
        loop {
            let id = match self.advance() {
                Token::Identifier(name) => name.clone(),
                token => return Err(Error::ParseError(format!("Expected identifier, got {:?}", token))),
            };
            
            let init = if self.peek() == &Token::Assign {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };
            
            declarations.push(VariableDeclarator { id, init });
            
            if self.peek() == &Token::Comma {
                self.advance();
            } else {
                break;
            }
        }
        
        self.expect(&Token::Semicolon)?;
        
        Ok(Statement::VariableDeclaration { kind, declarations })
    }
    
    fn parse_function_declaration(&mut self) -> Result<Statement> {
        self.expect(&Token::Function)?;
        
        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            token => return Err(Error::ParseError(format!("Expected function name, got {:?}", token))),
        };
        
        self.expect(&Token::LeftParen)?;
        let params = self.parse_params()?;
        self.expect(&Token::RightParen)?;
        
        self.expect(&Token::LeftBrace)?;
        let body = self.parse_block_body()?;
        self.expect(&Token::RightBrace)?;
        
        Ok(Statement::FunctionDeclaration {
            name,
            params,
            body,
            is_async: false,
        })
    }
    
    fn parse_params(&mut self) -> Result<Vec<String>> {
        let mut params = Vec::new();
        
        if self.peek() != &Token::RightParen {
            loop {
                let param = match self.advance() {
                    Token::Identifier(name) => name.clone(),
                    token => return Err(Error::ParseError(format!("Expected parameter name, got {:?}", token))),
                };
                params.push(param);
                
                if self.peek() == &Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        Ok(params)
    }
    
    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.expect(&Token::Return)?;
        
        let value = if self.peek() != &Token::Semicolon {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.expect(&Token::Semicolon)?;
        
        Ok(Statement::ReturnStatement(value))
    }
    
    fn parse_if_statement(&mut self) -> Result<Statement> {
        self.expect(&Token::If)?;
        self.expect(&Token::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(&Token::RightParen)?;
        
        let consequent = Box::new(self.parse_statement()?);
        
        let alternate = if self.peek() == &Token::Else {
            self.advance();
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        
        Ok(Statement::IfStatement {
            condition,
            consequent,
            alternate,
        })
    }
    
    fn parse_while_statement(&mut self) -> Result<Statement> {
        self.expect(&Token::While)?;
        self.expect(&Token::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(&Token::RightParen)?;
        
        let body = Box::new(self.parse_statement()?);
        
        Ok(Statement::WhileStatement { condition, body })
    }
    
    fn parse_block_statement(&mut self) -> Result<Statement> {
        self.expect(&Token::LeftBrace)?;
        let body = self.parse_block_body()?;
        self.expect(&Token::RightBrace)?;
        Ok(Statement::BlockStatement(body))
    }
    
    fn parse_block_body(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        
        while self.peek() != &Token::RightBrace && self.peek() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }
        
        Ok(statements)
    }
    
    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;
        self.expect(&Token::Semicolon)?;
        Ok(Statement::Expression(expr))
    }
    
    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<Expression> {
        let left = self.parse_or()?;
        
        if self.peek() == &Token::Assign {
            self.advance();
            let value = self.parse_assignment()?;
            Ok(Expression::Assignment {
                target: Box::new(left),
                value: Box::new(value),
            })
        } else {
            Ok(left)
        }
    }
    
    fn parse_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_and()?;
        
        while self.peek() == &Token::Or {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::BinaryOp {
                op: BinaryOperator::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_equality()?;
        
        while self.peek() == &Token::And {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::BinaryOp {
                op: BinaryOperator::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_equality(&mut self) -> Result<Expression> {
        let mut left = self.parse_comparison()?;
        
        loop {
            let op = match self.peek() {
                Token::Equal => Some(BinaryOperator::Eq),
                Token::StrictEqual => Some(BinaryOperator::StrictEq),
                Token::NotEqual => Some(BinaryOperator::NotEqual),
                Token::StrictNotEqual => Some(BinaryOperator::StrictNotEqual),
                _ => None,
            };
            
            if let Some(op) = op {
                self.advance();
                let right = self.parse_comparison()?;
                left = Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut left = self.parse_additive()?;
        
        loop {
            let op = match self.peek() {
                Token::Less => Some(BinaryOperator::Less),
                Token::Greater => Some(BinaryOperator::Greater),
                Token::LessEqual => Some(BinaryOperator::LessEqual),
                Token::GreaterEqual => Some(BinaryOperator::GreaterEqual),
                _ => None,
            };
            
            if let Some(op) = op {
                self.advance();
                let right = self.parse_additive()?;
                left = Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_additive(&mut self) -> Result<Expression> {
        let mut left = self.parse_multiplicative()?;
        
        loop {
            let op = match self.peek() {
                Token::Plus => Some(BinaryOperator::Add),
                Token::Minus => Some(BinaryOperator::Sub),
                _ => None,
            };
            
            if let Some(op) = op {
                self.advance();
                let right = self.parse_multiplicative()?;
                left = Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary()?;
        
        loop {
            let op = match self.peek() {
                Token::Star => Some(BinaryOperator::Mul),
                Token::Slash => Some(BinaryOperator::Div),
                Token::Percent => Some(BinaryOperator::Mod),
                _ => None,
            };
            
            if let Some(op) = op {
                self.advance();
                let right = self.parse_unary()?;
                left = Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand: Box::new(operand),
                })
            }
            Token::Not => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::UnaryOp {
                    op: UnaryOperator::Not,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_call(),
        }
    }
    
    fn parse_call(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary()?;
        
        loop {
            if self.peek() == &Token::LeftParen {
                self.advance();
                let args = self.parse_args()?;
                self.expect(&Token::RightParen)?;
                expr = Expression::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.peek() == &Token::Dot {
                self.advance();
                let property = match self.advance() {
                    Token::Identifier(name) => Expression::Identifier(name.clone()),
                    token => return Err(Error::ParseError(format!("Expected property name, got {:?}", token))),
                };
                expr = Expression::Member {
                    object: Box::new(expr),
                    property: Box::new(property),
                    computed: false,
                };
            } else if self.peek() == &Token::LeftBracket {
                self.advance();
                let property = self.parse_expression()?;
                self.expect(&Token::RightBracket)?;
                expr = Expression::Member {
                    object: Box::new(expr),
                    property: Box::new(property),
                    computed: true,
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn parse_args(&mut self) -> Result<Vec<Expression>> {
        let mut args = Vec::new();
        
        if self.peek() != &Token::RightParen {
            loop {
                args.push(self.parse_expression()?);
                if self.peek() == &Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        Ok(args)
    }
    
    fn parse_primary(&mut self) -> Result<Expression> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expression::NumberLiteral(n))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expression::StringLiteral(s))
            }
            Token::Identifier(name) => {
                self.advance();
                match name.as_str() {
                    "true" => Ok(Expression::BooleanLiteral(true)),
                    "false" => Ok(Expression::BooleanLiteral(false)),
                    "null" => Ok(Expression::NullLiteral),
                    "undefined" => Ok(Expression::UndefinedLiteral),
                    _ => Ok(Expression::Identifier(name)),
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&Token::RightParen)?;
                Ok(expr)
            }
            Token::Function => {
                self.advance();
                
                let name = if let Token::Identifier(name) = self.peek() {
                    let name = name.clone();
                    self.advance();
                    Some(name)
                } else {
                    None
                };
                
                self.expect(&Token::LeftParen)?;
                let params = self.parse_params()?;
                self.expect(&Token::RightParen)?;
                
                self.expect(&Token::LeftBrace)?;
                let body = self.parse_block_body()?;
                self.expect(&Token::RightBrace)?;
                
                Ok(Expression::FunctionExpression {
                    name,
                    params,
                    body,
                })
            }
            token => Err(Error::ParseError(format!("Unexpected token {:?}", token))),
        }
    }
}
