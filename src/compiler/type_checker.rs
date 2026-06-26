use crate::compiler::parser::{AstNode, Statement, Expression, BinaryOperator};
use crate::errors::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Null,
    Undefined,
    Void,
    Never,
    Unknown,
    Any,
    Union(Vec<Type>),
    Intersection(Vec<Type>),
}

pub struct TypeChecker {
    scopes: Vec<Vec<(String, Type)>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![Vec::new()],
        }
    }
    
    pub fn check(ast: &AstNode) -> Result<()> {
        let mut checker = Self::new();
        checker.check_node(ast)
    }
    
    fn check_node(&mut self, node: &AstNode) -> Result<()> {
        match node {
            AstNode::Program(statements) => {
                for stmt in statements {
                    self.check_statement(stmt)?;
                }
                Ok(())
            }
            _ => Err(Error::InternalError("Invalid root node".into())),
        }
    }
    
    fn check_statement(&mut self, stmt: &Statement) -> Result<Type> {
        match stmt {
            Statement::Expression(expr) => {
                self.check_expression(expr)?;
                Ok(Type::Void)
            }
            Statement::VariableDeclaration { kind, declarations } => {
                for decl in declarations {
                    if let Some(init) = &decl.init {
                        let init_type = self.check_expression(init)?;
                        
                        if let Some(declared_type) = self.get_variable_type(&decl.id) {
                            if !self.is_compatible(&declared_type, &init_type) {
                                return Err(Error::TypeError(format!(
                                    "Type '{:?}' is not assignable to type '{:?}'",
                                    init_type, declared_type
                                )));
                            }
                        } else {
                            self.define_variable(&decl.id, init_type);
                        }
                    } else if matches!(kind, crate::compiler::parser::VarKind::Const) {
                        return Err(Error::TypeError("Missing initializer in const declaration".into()));
                    }
                }
                Ok(Type::Void)
            }
            Statement::ReturnStatement(value) => {
                if let Some(expr) = value {
                    self.check_expression(expr)?;
                }
                Ok(Type::Void)
            }
            Statement::IfStatement { condition, consequent, alternate } => {
                self.check_expression(condition)?;
                self.check_statement(consequent)?;
                if let Some(alt) = alternate {
                    self.check_statement(alt)?;
                }
                Ok(Type::Void)
            }
            Statement::WhileStatement { condition, body } => {
                self.check_expression(condition)?;
                self.check_statement(body)?;
                Ok(Type::Void)
            }
            Statement::BlockStatement(stmts) => {
                self.enter_scope();
                for stmt in stmts {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();
                Ok(Type::Void)
            }
            Statement::FunctionDeclaration { name, params, body, .. } => {
                self.enter_scope();
                for param in params {
                    self.define_variable(param, Type::Any);
                }
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();
                self.define_variable(name, Type::Any);
                Ok(Type::Void)
            }
        }
    }
    
    fn check_expression(&mut self, expr: &Expression) -> Result<Type> {
        match expr {
            Expression::NumberLiteral(_) => Ok(Type::Number),
            Expression::StringLiteral(_) => Ok(Type::String),
            Expression::BooleanLiteral(_) => Ok(Type::Boolean),
            Expression::NullLiteral => Ok(Type::Null),
            Expression::UndefinedLiteral => Ok(Type::Undefined),
            Expression::Identifier(name) => {
                self.get_variable_type(name)
                    .ok_or_else(|| Error::ReferenceError(format!("'{}' is not defined", name)))
            }
            Expression::BinaryOp { op, left, right } => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;
                self.check_binary_op(op, &left_type, &right_type)
            }
            Expression::UnaryOp { op, operand } => {
                let operand_type = self.check_expression(operand)?;
                self.check_unary_op(op, &operand_type)
            }
            Expression::Assignment { target, value } => {
                let target_type = self.check_expression(target)?;
                let value_type = self.check_expression(value)?;
                if !self.is_compatible(&target_type, &value_type) {
                    return Err(Error::TypeError(format!(
                        "Type '{:?}' is not assignable to type '{:?}'",
                        value_type, target_type
                    )));
                }
                Ok(target_type)
            }
            Expression::Call { callee, args } => {
                for arg in args {
                    self.check_expression(arg)?;
                }
                self.check_expression(callee)?;
                Ok(Type::Any)
            }
            Expression::Member { object, .. } => {
                self.check_expression(object)?;
                Ok(Type::Any)
            }
            Expression::FunctionExpression { params, body, .. } => {
                self.enter_scope();
                for param in params {
                    self.define_variable(param, Type::Any);
                }
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();
                Ok(Type::Any)
            }
        }
    }
    
    fn check_binary_op(&self, op: &BinaryOperator, left: &Type, right: &Type) -> Result<Type> {
        match op {
            BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul |
            BinaryOperator::Div | BinaryOperator::Mod => {
                if matches!(left, Type::Number) && matches!(right, Type::Number) {
                    Ok(Type::Number)
                } else if matches!(op, BinaryOperator::Add) && matches!(left, Type::String) && matches!(right, Type::String) {
                    Ok(Type::String)
                } else {
                    Err(Error::TypeError(format!(
                        "Operator '{:?}' cannot be applied to types '{:?}' and '{:?}'",
                        op, left, right
                    )))
                }
            }
            BinaryOperator::Eq | BinaryOperator::StrictEq |
            BinaryOperator::NotEqual | BinaryOperator::StrictNotEqual |
            BinaryOperator::Less | BinaryOperator::Greater |
            BinaryOperator::LessEqual | BinaryOperator::GreaterEqual => {
                Ok(Type::Boolean)
            }
            BinaryOperator::And | BinaryOperator::Or => {
                if matches!(left, Type::Boolean) && matches!(right, Type::Boolean) {
                    Ok(Type::Boolean)
                } else {
                    Ok(Type::Any)
                }
            }
            _ => Ok(Type::Number),
        }
    }
    
    fn check_unary_op(&self, op: &crate::compiler::parser::UnaryOperator, operand: &Type) -> Result<Type> {
        match op {
            crate::compiler::parser::UnaryOperator::Negate => {
                if matches!(operand, Type::Number) {
                    Ok(Type::Number)
                } else {
                    Err(Error::TypeError(format!(
                        "Operator '-' cannot be applied to type '{:?}'",
                        operand
                    )))
                }
            }
            crate::compiler::parser::UnaryOperator::Not => Ok(Type::Boolean),
            _ => Ok(Type::Any),
        }
    }
    
    fn enter_scope(&mut self) {
        self.scopes.push(Vec::new());
    }
    
    fn exit_scope(&mut self) {
        self.scopes.pop();
    }
    
    fn define_variable(&mut self, name: &str, type_: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.push((name.to_string(), type_));
        }
    }
    
    fn get_variable_type(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            for (var_name, var_type) in scope {
                if var_name == name {
                    return Some(var_type.clone());
                }
            }
        }
        None
    }
    
    fn is_compatible(&self, expected: &Type, actual: &Type) -> bool {
        if expected == actual {
            return true;
        }
        
        if matches!(expected, Type::Any) || matches!(actual, Type::Any) {
            return true;
        }
        
        match (expected, actual) {
            (Type::Union(types), _) => types.iter().any(|t| self.is_compatible(t, actual)),
            (_, Type::Union(types)) => types.iter().any(|t| self.is_compatible(expected, t)),
            _ => false,
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
