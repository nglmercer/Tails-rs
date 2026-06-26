use crate::compiler::parser::{AstNode, Statement, Expression, BinaryOperator, VarKind};
use crate::errors::{Error, Result};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    NumberLiteral(f64),
    String,
    StringLiteral(String),
    Boolean,
    BooleanLiteral(bool),
    Null,
    Undefined,
    Void,
    Never,
    Unknown,
    Any,
    Union(Vec<Type>),
    Intersection(Vec<Type>),
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Object(Vec<ObjectProperty>),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Named(String),
    Generic {
        name: String,
        args: Vec<Type>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectProperty {
    pub name: String,
    pub ty: Type,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct InterfaceDecl {
    pub name: String,
    pub extends: Vec<String>,
    pub properties: Vec<ObjectProperty>,
}

#[derive(Debug, Clone)]
pub struct TypeAliasDecl {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub members: Vec<(String, Type)>,
}

pub struct TypeChecker {
    scopes: Vec<HashMap<String, Type>>,
    interfaces: HashMap<String, InterfaceDecl>,
    type_aliases: HashMap<String, TypeAliasDecl>,
    enums: HashMap<String, EnumDecl>,
    narrowed_types: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            interfaces: HashMap::new(),
            type_aliases: HashMap::new(),
            enums: HashMap::new(),
            narrowed_types: HashMap::new(),
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

                        if let Some(ann) = &decl.type_annotation {
                            let declared = self.resolve_annotation(ann)?;
                            if !self.is_compatible(&declared, &init_type) {
                                return Err(Error::TypeError(format!(
                                    "Type '{:?}' is not assignable to type '{:?}'",
                                    init_type, declared
                                )));
                            }
                            self.define_variable(&decl.id, declared);
                        } else {
                            self.define_variable(&decl.id, init_type);
                        }
                    } else if matches!(kind, VarKind::Const) {
                        return Err(Error::TypeError(
                            "Missing initializer in const declaration".into(),
                        ));
                    } else if let Some(ann) = &decl.type_annotation {
                        let declared = self.resolve_annotation(ann)?;
                        self.define_variable(&decl.id, declared);
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
            Statement::IfStatement {
                condition,
                consequent,
                alternate,
            } => {
                let cond_type = self.check_expression(condition)?;
                let saved = self.narrowed_types.clone();
                self.apply_type_narrowing(condition, &cond_type, true);
                self.check_statement(consequent)?;
                let after_then = self.narrowed_types.clone();
                self.narrowed_types = saved;
                self.apply_type_narrowing(condition, &cond_type, false);
                if let Some(alt) = alternate {
                    self.check_statement(alt)?;
                }
                self.narrowed_types = after_then;
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
            Statement::FunctionDeclaration {
                name,
                params,
                param_types,
                return_type,
                body,
                ..
            } => {
                let param_tys: Vec<Type> = if let Some(pts) = param_types {
                    pts.iter()
                        .map(|pt| {
                            if let Some(ann) = pt {
                                self.resolve_annotation(ann)
                            } else {
                                Ok(Type::Any)
                            }
                        })
                        .collect::<Result<Vec<_>>>()?
                } else {
                    params.iter().map(|_| Type::Any).collect()
                };

                let ret_ty = if let Some(rt) = return_type {
                    self.resolve_annotation(rt)?
                } else {
                    Type::Any
                };

                let func_type = Type::Function {
                    params: param_tys.clone(),
                    return_type: Box::new(ret_ty),
                };
                self.define_variable(name, func_type);

                self.enter_scope();
                for (param, pty) in params.iter().zip(param_tys.iter()) {
                    self.define_variable(param, pty.clone());
                }
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();
                Ok(Type::Void)
            }
            Statement::InterfaceDeclaration {
                name,
                extends,
                members,
            } => {
                let mut properties = Vec::new();
                for member in members {
                    match member {
                        crate::compiler::parser::InterfaceMember::Property {
                            name,
                            type_annotation,
                            optional,
                        } => {
                            let ty = self.resolve_annotation(type_annotation)?;
                            properties.push(ObjectProperty {
                                name: name.clone(),
                                ty,
                                optional: *optional,
                            });
                        }
                        crate::compiler::parser::InterfaceMember::Method {
                            name,
                            params: _,
                            return_type: _,
                        } => {
                            properties.push(ObjectProperty {
                                name: name.clone(),
                                ty: Type::Any,
                                optional: false,
                            });
                        }
                    }
                }
                self.interfaces.insert(
                    name.clone(),
                    InterfaceDecl {
                        name: name.clone(),
                        extends: extends.clone(),
                        properties,
                    },
                );
                self.define_variable(name, Type::Any);
                Ok(Type::Void)
            }
            Statement::TypeAliasDeclaration {
                name,
                type_annotation,
            } => {
                let ty = self.resolve_annotation(type_annotation)?;
                self.type_aliases.insert(
                    name.clone(),
                    TypeAliasDecl {
                        name: name.clone(),
                        ty: ty.clone(),
                    },
                );
                Ok(Type::Void)
            }
            Statement::EnumDeclaration { name, members } => {
                let mut enum_members = Vec::new();
                let mut value = 0.0;
                for member in members {
                    let member_type = if let Some(val) = &member.value {
                        match val {
                            crate::compiler::parser::TypeLiteral::Number(n) => {
                                value = *n;
                                Type::NumberLiteral(*n)
                            }
                            crate::compiler::parser::TypeLiteral::String(s) => {
                                Type::StringLiteral(s.clone())
                            }
                            crate::compiler::parser::TypeLiteral::Boolean(b) => {
                                Type::BooleanLiteral(*b)
                            }
                        }
                    } else {
                        let t = Type::NumberLiteral(value);
                        value += 1.0;
                        t
                    };
                    enum_members.push((member.name.clone(), member_type));
                }
                self.enums.insert(
                    name.clone(),
                    EnumDecl {
                        name: name.clone(),
                        members: enum_members,
                    },
                );
                self.define_variable(name, Type::Any);
                Ok(Type::Void)
            }
            Statement::ClassDeclaration {
                name,
                superclass: _,
                body,
            } => {
                self.define_variable(name, Type::Any);
                self.enter_scope();
                for member in body {
                    match member {
                        crate::compiler::parser::ClassMember::Method {
                            name,
                            params,
                            body,
                            ..
                        } => {
                            self.enter_scope();
                            for param in params {
                                self.define_variable(param, Type::Any);
                            }
                            for stmt in body {
                                self.check_statement(stmt)?;
                            }
                            self.exit_scope();
                        }
                        crate::compiler::parser::ClassMember::Constructor { params, body } => {
                            self.enter_scope();
                            for param in params {
                                self.define_variable(param, Type::Any);
                            }
                            for stmt in body {
                                self.check_statement(stmt)?;
                            }
                            self.exit_scope();
                        }
                        _ => {}
                    }
                }
                self.exit_scope();
                Ok(Type::Void)
            }
            Statement::ForStatement {
                init,
                condition,
                update,
                body,
            } => {
                self.enter_scope();
                if let Some(init) = init {
                    match init.as_ref() {
                        crate::compiler::parser::ForInit::Variable(stmt) => {
                            self.check_statement(stmt)?;
                        }
                        crate::compiler::parser::ForInit::Expression(expr) => {
                            self.check_expression(expr)?;
                        }
                    }
                }
                if let Some(cond) = condition {
                    self.check_expression(cond)?;
                }
                if let Some(upd) = update {
                    self.check_expression(upd)?;
                }
                self.check_statement(body)?;
                self.exit_scope();
                Ok(Type::Void)
            }
            Statement::ForInStatement {
                left: _,
                right,
                body,
            } => {
                self.check_expression(right)?;
                self.check_statement(body)?;
                Ok(Type::Void)
            }
            Statement::ForOfStatement {
                left: _,
                right,
                body,
                ..
            } => {
                self.check_expression(right)?;
                self.check_statement(body)?;
                Ok(Type::Void)
            }
            Statement::SwitchStatement {
                discriminant,
                cases,
            } => {
                self.check_expression(discriminant)?;
                for case in cases {
                    if let Some(test) = &case.test {
                        self.check_expression(test)?;
                    }
                    for stmt in &case.consequent {
                        self.check_statement(stmt)?;
                    }
                }
                Ok(Type::Void)
            }
            Statement::TryStatement {
                block,
                handler,
                finalizer,
            } => {
                self.enter_scope();
                for stmt in block {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();
                if let Some(catch) = handler {
                    self.enter_scope();
                    self.define_variable(&catch.param, Type::Any);
                    for stmt in &catch.body {
                        self.check_statement(stmt)?;
                    }
                    self.exit_scope();
                }
                if let Some(fin) = finalizer {
                    for stmt in fin {
                        self.check_statement(stmt)?;
                    }
                }
                Ok(Type::Void)
            }
            Statement::ThrowStatement(expr) => {
                self.check_expression(expr)?;
                Ok(Type::Never)
            }
            Statement::ExportDeclaration { declaration } => {
                self.check_statement(declaration)
            }
            Statement::ExportDefaultDeclaration { declaration } => {
                self.check_statement(declaration)
            }
            Statement::BreakStatement | Statement::ContinueStatement => Ok(Type::Void),
            Statement::ImportDeclaration { .. } => Ok(Type::Void),
            Statement::DoWhileStatement { condition, body } => {
                self.check_expression(condition)?;
                self.check_statement(body)?;
                Ok(Type::Void)
            }
        }
    }

    fn check_expression(&mut self, expr: &Expression) -> Result<Type> {
        match expr {
            Expression::NumberLiteral(n) => Ok(Type::NumberLiteral(*n)),
            Expression::StringLiteral(s) => Ok(Type::StringLiteral(s.clone())),
            Expression::BooleanLiteral(b) => Ok(Type::BooleanLiteral(*b)),
            Expression::NullLiteral => Ok(Type::Null),
            Expression::UndefinedLiteral => Ok(Type::Undefined),
            Expression::Identifier(name) => {
                if let Some(ty) = self.narrowed_types.get(name) {
                    return Ok(ty.clone());
                }
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
            Expression::Assignment { target, value, .. } => {
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
                let callee_type = self.check_expression(callee)?;
                let arg_types: Vec<Type> = args
                    .iter()
                    .map(|a| self.check_expression(a))
                    .collect::<Result<Vec<_>>>()?;

                if let Type::Function {
                    params,
                    return_type,
                } = callee_type
                {
                    for (i, (param_ty, arg_ty)) in
                        params.iter().zip(arg_types.iter()).enumerate()
                    {
                        if !self.is_compatible(param_ty, arg_ty) {
                            return Err(Error::TypeError(format!(
                                "Argument {} type mismatch: expected {:?}, got {:?}",
                                i + 1, param_ty, arg_ty
                            )));
                        }
                    }
                    Ok(*return_type)
                } else {
                    Ok(Type::Any)
                }
            }
            Expression::Member {
                object,
                property,
                computed,
            } => {
                let obj_type = self.check_expression(object)?;
                if *computed {
                    self.check_expression(property)?;
                }
                match &obj_type {
                    Type::Object(props) => {
                        if let Expression::Identifier(name) = property.as_ref() {
                            if !*computed {
                                for prop in props {
                                    if prop.name == *name {
                                        return Ok(prop.ty.clone());
                                    }
                                }
                            }
                        }
                        Ok(Type::Any)
                    }
                    Type::Array(_) => {
                        if let Expression::Identifier(name) = property.as_ref() {
                            if !*computed && name == "length" {
                                return Ok(Type::Number);
                            }
                        }
                        Ok(Type::Any)
                    }
                    Type::Named(name) => {
                        if let Some(iface) = self.interfaces.get(name) {
                            if let Expression::Identifier(prop_name) = property.as_ref() {
                                if !*computed {
                                    for prop in &iface.properties {
                                        if prop.name == *prop_name {
                                            return Ok(prop.ty.clone());
                                        }
                                    }
                                }
                            }
                        }
                        Ok(Type::Any)
                    }
                    _ => Ok(Type::Any),
                }
            }
            Expression::FunctionExpression {
                params,
                param_types,
                return_type,
                body,
                ..
            } => {
                let param_tys: Vec<Type> = if let Some(pts) = param_types {
                    pts.iter()
                        .map(|pt| {
                            if let Some(ann) = pt {
                                self.resolve_annotation(ann)
                            } else {
                                Ok(Type::Any)
                            }
                        })
                        .collect::<Result<Vec<_>>>()?
                } else {
                    params.iter().map(|_| Type::Any).collect()
                };

                let ret_ty = if let Some(rt) = return_type {
                    self.resolve_annotation(rt)?
                } else {
                    Type::Any
                };

                self.enter_scope();
                for (param, pty) in params.iter().zip(param_tys.iter()) {
                    self.define_variable(param, pty.clone());
                }
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();
                Ok(Type::Function {
                    params: param_tys,
                    return_type: Box::new(ret_ty),
                })
            }
            Expression::ArrowFunction {
                params,
                body,
                param_types,
                return_type,
                ..
            } => {
                let param_tys: Vec<Type> = if let Some(pts) = param_types {
                    pts.iter()
                        .map(|pt| {
                            if let Some(ann) = pt {
                                self.resolve_annotation(ann)
                            } else {
                                Ok(Type::Any)
                            }
                        })
                        .collect::<Result<Vec<_>>>()?
                } else {
                    params.iter().map(|_| Type::Any).collect()
                };

                let ret_ty = if let Some(rt) = return_type {
                    self.resolve_annotation(rt)?
                } else {
                    Type::Any
                };

                self.enter_scope();
                for (param, pty) in params.iter().zip(param_tys.iter()) {
                    self.define_variable(param, pty.clone());
                }
                match body.as_ref() {
                    crate::compiler::parser::ArrowFunctionBody::Expression(expr) => {
                        self.check_expression(expr)?;
                    }
                    crate::compiler::parser::ArrowFunctionBody::Block(stmts) => {
                        for stmt in stmts {
                            self.check_statement(stmt)?;
                        }
                    }
                }
                self.exit_scope();
                Ok(Type::Function {
                    params: param_tys,
                    return_type: Box::new(ret_ty),
                })
            }
            Expression::ArrayLiteral { elements } => {
                if elements.is_empty() {
                    return Ok(Type::Array(Box::new(Type::Any)));
                }
                let elem_types: Vec<Type> = elements
                    .iter()
                    .map(|e| self.check_expression(e))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Type::Tuple(elem_types))
            }
            Expression::ObjectLiteral { properties } => {
                let mut obj_props = Vec::new();
                for (key, value) in properties {
                    let val_type = self.check_expression(value)?;
                    obj_props.push(ObjectProperty {
                        name: key.clone(),
                        ty: val_type,
                        optional: false,
                    });
                }
                Ok(Type::Object(obj_props))
            }
            Expression::TypeAssertion {
                expression,
                type_annotation,
            } => {
                self.check_expression(expression)?;
                self.resolve_annotation(type_annotation)
            }
            Expression::ConditionalExpression {
                test,
                consequent,
                alternate,
            } => {
                self.check_expression(test)?;
                let cons_type = self.check_expression(consequent)?;
                let alt_type = self.check_expression(alternate)?;
                if self.is_compatible(&cons_type, &alt_type) {
                    Ok(cons_type)
                } else {
                    Ok(Type::Union(vec![cons_type, alt_type]))
                }
            }
            Expression::TemplateLiteral {
                expressions, ..
            } => {
                for expr in expressions {
                    self.check_expression(expr)?;
                }
                Ok(Type::String)
            }
            Expression::NewExpression { callee, args } => {
                self.check_expression(callee)?;
                for arg in args {
                    self.check_expression(arg)?;
                }
                Ok(Type::Any)
            }
            Expression::UpdateExpression { operand, .. } => {
                let ty = self.check_expression(operand)?;
                if matches!(ty, Type::Number) || matches!(ty, Type::NumberLiteral(_)) {
                    Ok(Type::Number)
                } else {
                    Ok(Type::Number)
                }
            }
            Expression::AwaitExpression { argument } => {
                self.check_expression(argument)?;
                Ok(Type::Any)
            }
            Expression::SuperCall { args } => {
                for arg in args {
                    self.check_expression(arg)?;
                }
                Ok(Type::Any)
            }
            Expression::SuperMember { .. } => Ok(Type::Any),
            Expression::ClassExpression {
                name,
                superclass: _,
                body,
            } => {
                if let Some(n) = name {
                    self.define_variable(n, Type::Any);
                }
                self.enter_scope();
                for member in body {
                    if let crate::compiler::parser::ClassMember::Method {
                        params,
                        body,
                        ..
                    } = member
                    {
                        self.enter_scope();
                        for param in params {
                            self.define_variable(param, Type::Any);
                        }
                        for stmt in body {
                            self.check_statement(stmt)?;
                        }
                        self.exit_scope();
                    }
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
                if self.is_number_type(left) && self.is_number_type(right) {
                    Ok(Type::Number)
                } else if matches!(op, BinaryOperator::Add) && self.is_string_type(left) && self.is_string_type(right) {
                    Ok(Type::String)
                } else if matches!(op, BinaryOperator::Add)
                    && (self.is_number_type(left) || self.is_string_type(left))
                    && (self.is_number_type(right) || self.is_string_type(right))
                {
                    Ok(Type::Any)
                } else {
                    Err(Error::TypeError(format!(
                        "Operator '{:?}' cannot be applied to types '{:?}' and '{:?}'",
                        op, left, right
                    )))
                }
            }
            BinaryOperator::Eq | BinaryOperator::StrictEq | BinaryOperator::NotEqual |
            BinaryOperator::StrictNotEqual | BinaryOperator::Less | BinaryOperator::Greater |
            BinaryOperator::LessEqual | BinaryOperator::GreaterEqual => Ok(Type::Boolean),
            BinaryOperator::And | BinaryOperator::Or => {
                if matches!(left, Type::Boolean) || matches!(right, Type::Boolean) {
                    Ok(Type::Boolean)
                } else {
                    Ok(Type::Any)
                }
            }
            BinaryOperator::Power => {
                if self.is_number_type(left) && self.is_number_type(right) {
                    Ok(Type::Number)
                } else {
                    Err(Error::TypeError(format!(
                        "Operator '{:?}' cannot be applied to types '{:?}' and '{:?}'",
                        op, left, right
                    )))
                }
            }
            BinaryOperator::Instanceof | BinaryOperator::In => Ok(Type::Boolean),
            BinaryOperator::BitAnd | BinaryOperator::BitOr | BinaryOperator::BitXor |
            BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => Ok(Type::Number),
        }
    }

    fn check_unary_op(
        &self,
        op: &crate::compiler::parser::UnaryOperator,
        operand: &Type,
    ) -> Result<Type> {
        match op {
            crate::compiler::parser::UnaryOperator::Negate => {
                if self.is_number_type(operand) {
                    Ok(Type::Number)
                } else {
                    Err(Error::TypeError(format!(
                        "Operator '-' cannot be applied to type '{:?}'",
                        operand
                    )))
                }
            }
            crate::compiler::parser::UnaryOperator::Not => Ok(Type::Boolean),
            crate::compiler::parser::UnaryOperator::Typeof => Ok(Type::String),
            crate::compiler::parser::UnaryOperator::Void => Ok(Type::Undefined),
            crate::compiler::parser::UnaryOperator::Delete => Ok(Type::Boolean),
            crate::compiler::parser::UnaryOperator::BitNot => Ok(Type::Number),
        }
    }

    fn apply_type_narrowing(
        &mut self,
        condition: &Expression,
        cond_type: &Type,
        is_true_branch: bool,
    ) {
        if let Expression::BinaryOp {
            op,
            left,
            right,
        } = condition
        {
            if matches!(op, BinaryOperator::StrictEq) || matches!(op, BinaryOperator::Eq) {
                if let (Expression::UnaryOp { op: crate::compiler::parser::UnaryOperator::Typeof, operand }, Expression::StringLiteral(lit)) =
                    (left.as_ref(), right.as_ref())
                {
                    if let Expression::Identifier(name) = operand.as_ref() {
                        if let Some(orig) = self.get_variable_type(name) {
                            if let Type::Union(variants) = &orig {
                                let narrowed: Vec<Type> = variants
                                    .iter()
                                    .filter(|v| self.type_matches_typeof_value(v, lit))
                                    .cloned()
                                    .collect();
                                if !narrowed.is_empty() {
                                    let ty = if narrowed.len() == 1 {
                                        narrowed[0].clone()
                                    } else {
                                        Type::Union(narrowed)
                                    };
                                    if is_true_branch {
                                        self.narrowed_types.insert(name.clone(), ty);
                                    } else {
                                        let excluded: Vec<Type> = variants
                                            .iter()
                                            .filter(|v| !self.type_matches_typeof_value(v, lit))
                                            .cloned()
                                            .collect();
                                        if !excluded.is_empty() {
                                            let ty = if excluded.len() == 1 {
                                                excluded[0].clone()
                                            } else {
                                                Type::Union(excluded)
                                            };
                                            self.narrowed_types.insert(name.clone(), ty);
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let (Expression::Identifier(name), Expression::StringLiteral(lit)) =
                    (left.as_ref(), right.as_ref())
                {
                    if is_true_branch {
                        self.narrowed_types
                            .insert(name.clone(), Type::StringLiteral(lit.clone()));
                    } else if let Some(orig) = self.get_variable_type(name) {
                        self.narrowed_types.insert(name.clone(), orig);
                    }
                } else if let (Expression::Identifier(name), Expression::NumberLiteral(n)) =
                    (left.as_ref(), right.as_ref())
                {
                    if is_true_branch {
                        self.narrowed_types
                            .insert(name.clone(), Type::NumberLiteral(*n));
                    } else if let Some(orig) = self.get_variable_type(name) {
                        self.narrowed_types.insert(name.clone(), orig);
                    }
                } else if let (Expression::Identifier(name), Expression::BooleanLiteral(b)) =
                    (left.as_ref(), right.as_ref())
                {
                    if is_true_branch {
                        self.narrowed_types
                            .insert(name.clone(), Type::BooleanLiteral(*b));
                    } else if let Some(orig) = self.get_variable_type(name) {
                        self.narrowed_types.insert(name.clone(), orig);
                    }
                } else if let (Expression::Identifier(name), Expression::NullLiteral) =
                    (left.as_ref(), right.as_ref())
                {
                    if is_true_branch {
                        self.narrowed_types
                            .insert(name.clone(), Type::Null);
                    } else if let Some(orig) = self.get_variable_type(name) {
                        self.narrowed_types.insert(name.clone(), orig);
                    }
                }
            } else if matches!(op, BinaryOperator::StrictNotEqual) || matches!(op, BinaryOperator::NotEqual) {
                if let (Expression::Identifier(name), Expression::NullLiteral) =
                    (left.as_ref(), right.as_ref())
                {
                    if is_true_branch {
                        if let Some(orig) = self.get_variable_type(name) {
                            let narrowed = self.remove_null_from_type(&orig);
                            self.narrowed_types.insert(name.clone(), narrowed);
                        }
                    }
                } else if let (Expression::Identifier(name), Expression::UndefinedLiteral) =
                    (left.as_ref(), right.as_ref())
                {
                    if is_true_branch {
                        if let Some(orig) = self.get_variable_type(name) {
                            let narrowed = self.remove_undefined_from_type(&orig);
                            self.narrowed_types.insert(name.clone(), narrowed);
                        }
                    }
                }
            }
        }

        if let Expression::UnaryOp { op, operand } = condition {
            if matches!(op, crate::compiler::parser::UnaryOperator::Typeof) {
                if let Expression::Identifier(name) = operand.as_ref() {
                    if let Some(orig) = self.get_variable_type(name) {
                        if let Type::Union(variants) = &orig {
                            if is_true_branch {
                                let narrowed: Vec<Type> = variants
                                    .iter()
                                    .filter(|v| {
                                        self.type_matches_typeof(v, condition)
                                    })
                                    .cloned()
                                    .collect();
                                if !narrowed.is_empty() {
                                    let ty = if narrowed.len() == 1 {
                                        narrowed[0].clone()
                                    } else {
                                        Type::Union(narrowed)
                                    };
                                    self.narrowed_types.insert(name.clone(), ty);
                                }
                            } else {
                                let narrowed: Vec<Type> = variants
                                    .iter()
                                    .filter(|v| {
                                        !self.type_matches_typeof(v, condition)
                                    })
                                    .cloned()
                                    .collect();
                                if !narrowed.is_empty() {
                                    let ty = if narrowed.len() == 1 {
                                        narrowed[0].clone()
                                    } else {
                                        Type::Union(narrowed)
                                    };
                                    self.narrowed_types.insert(name.clone(), ty);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn type_matches_typeof_value(&self, ty: &Type, typeof_str: &str) -> bool {
        match typeof_str {
            "number" => matches!(ty, Type::Number | Type::NumberLiteral(_)),
            "string" => matches!(ty, Type::String | Type::StringLiteral(_)),
            "boolean" => matches!(ty, Type::Boolean | Type::BooleanLiteral(_)),
            "object" => matches!(ty, Type::Object(_) | Type::Null | Type::Array(_) | Type::Tuple(_)),
            "function" => matches!(ty, Type::Function { .. }),
            "undefined" => matches!(ty, Type::Undefined),
            "bigint" => false,
            _ => true,
        }
    }

    fn type_matches_typeof(&self, ty: &Type, _typeof_expr: &Expression) -> bool {
        match ty {
            Type::Number | Type::NumberLiteral(_) => true,
            Type::String | Type::StringLiteral(_) => true,
            Type::Boolean | Type::BooleanLiteral(_) => true,
            Type::Null => true,
            Type::Undefined => true,
            _ => true,
        }
    }

    fn remove_null_from_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Union(variants) => {
                let filtered: Vec<Type> = variants
                    .iter()
                    .filter(|v| !matches!(v, Type::Null))
                    .cloned()
                    .collect();
                if filtered.len() == 1 {
                    filtered[0].clone()
                } else {
                    Type::Union(filtered)
                }
            }
            Type::Null => Type::Never,
            _ => ty.clone(),
        }
    }

    fn remove_undefined_from_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Union(variants) => {
                let filtered: Vec<Type> = variants
                    .iter()
                    .filter(|v| !matches!(v, Type::Undefined))
                    .cloned()
                    .collect();
                if filtered.len() == 1 {
                    filtered[0].clone()
                } else {
                    Type::Union(filtered)
                }
            }
            Type::Undefined => Type::Never,
            _ => ty.clone(),
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_variable(&mut self, name: &str, type_: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), type_);
        }
    }

    fn get_variable_type(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    fn resolve_annotation(
        &self,
        ann: &crate::compiler::parser::TypeAnnotation,
    ) -> Result<Type> {
        use crate::compiler::parser::TypeAnnotation;
        match ann {
            TypeAnnotation::Number => Ok(Type::Number),
            TypeAnnotation::String => Ok(Type::String),
            TypeAnnotation::Boolean => Ok(Type::Boolean),
            TypeAnnotation::Null => Ok(Type::Null),
            TypeAnnotation::Undefined => Ok(Type::Undefined),
            TypeAnnotation::Void => Ok(Type::Void),
            TypeAnnotation::Any => Ok(Type::Any),
            TypeAnnotation::Unknown => Ok(Type::Unknown),
            TypeAnnotation::Never => Ok(Type::Never),
            TypeAnnotation::Named(name) => {
                if self.type_aliases.contains_key(name) || self.interfaces.contains_key(name) || self.enums.contains_key(name) {
                    Ok(Type::Named(name.clone()))
                } else {
                    Ok(Type::Named(name.clone()))
                }
            }
            TypeAnnotation::Array(inner) => {
                let inner_ty = self.resolve_annotation(inner)?;
                Ok(Type::Array(Box::new(inner_ty)))
            }
            TypeAnnotation::Tuple(elements) => {
                let elem_tys: Vec<Type> = elements
                    .iter()
                    .map(|e| self.resolve_annotation(e))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Type::Tuple(elem_tys))
            }
            TypeAnnotation::Union(variants) => {
                let var_tys: Vec<Type> = variants
                    .iter()
                    .map(|v| self.resolve_annotation(v))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Type::Union(var_tys))
            }
            TypeAnnotation::Intersection(types) => {
                let tys: Vec<Type> = types
                    .iter()
                    .map(|t| self.resolve_annotation(t))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Type::Intersection(tys))
            }
            TypeAnnotation::Object(props) => {
                let mut obj_props = Vec::new();
                for (name, ty_ann, optional) in props {
                    let ty = self.resolve_annotation(ty_ann)?;
                    obj_props.push(ObjectProperty {
                        name: name.clone(),
                        ty,
                        optional: *optional,
                    });
                }
                Ok(Type::Object(obj_props))
            }
            TypeAnnotation::Function {
                params,
                return_type,
            } => {
                let param_tys: Vec<Type> = params
                    .iter()
                    .map(|p| self.resolve_annotation(p))
                    .collect::<Result<Vec<_>>>()?;
                let ret = self.resolve_annotation(return_type)?;
                Ok(Type::Function {
                    params: param_tys,
                    return_type: Box::new(ret),
                })
            }
            TypeAnnotation::Literal(lit) => match lit {
                crate::compiler::parser::TypeLiteral::Number(n) => Ok(Type::NumberLiteral(*n)),
                crate::compiler::parser::TypeLiteral::String(s) => Ok(Type::StringLiteral(s.clone())),
                crate::compiler::parser::TypeLiteral::Boolean(b) => Ok(Type::BooleanLiteral(*b)),
            },
            TypeAnnotation::Generic { name, args } => {
                let arg_tys: Vec<Type> = args
                    .iter()
                    .map(|a| self.resolve_annotation(a))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Type::Generic {
                    name: name.clone(),
                    args: arg_tys,
                })
            }
        }
    }

    fn is_number_type(&self, ty: &Type) -> bool {
        matches!(ty, Type::Number | Type::NumberLiteral(_))
    }

    fn is_string_type(&self, ty: &Type) -> bool {
        matches!(ty, Type::String | Type::StringLiteral(_))
    }

    fn is_boolean_type(&self, ty: &Type) -> bool {
        matches!(ty, Type::Boolean | Type::BooleanLiteral(_))
    }

    fn is_compatible(&self, expected: &Type, actual: &Type) -> bool {
        if expected == actual {
            return true;
        }

        if matches!(expected, Type::Any) || matches!(actual, Type::Any) {
            return true;
        }

        if matches!(expected, Type::Unknown) || matches!(actual, Type::Unknown) {
            return true;
        }

        if matches!(expected, Type::Never) {
            return true;
        }

        match (expected, actual) {
            (Type::Union(types), _) => types.iter().any(|t| self.is_compatible(t, actual)),
            (_, Type::Union(types)) => types.iter().any(|t| self.is_compatible(expected, t)),
            (Type::Number, Type::NumberLiteral(_)) => true,
            (Type::NumberLiteral(_), Type::Number) => true,
            (Type::String, Type::StringLiteral(_)) => true,
            (Type::StringLiteral(_), Type::String) => true,
            (Type::Boolean, Type::BooleanLiteral(_)) => true,
            (Type::BooleanLiteral(_), Type::Boolean) => true,
            (Type::Array(inner_exp), Type::Array(inner_act)) => {
                self.is_compatible(inner_exp, inner_act)
            }
            (Type::Tuple(t1), Type::Tuple(t2)) => {
                t1.len() == t2.len()
                    && t1.iter()
                        .zip(t2.iter())
                        .all(|(a, b)| self.is_compatible(a, b))
            }
            (Type::Tuple(t1), Type::Array(inner)) => {
                t1.iter().all(|t| self.is_compatible(t, inner))
            }
            (Type::Array(inner), Type::Tuple(t2)) => {
                t2.iter().all(|t| self.is_compatible(inner, t))
            }
            (Type::Object(props_exp), Type::Object(props_act)) => {
                for prop_exp in props_exp {
                    let found = props_act.iter().find(|p| p.name == prop_exp.name);
                    match found {
                        Some(prop_act) => {
                            if !prop_exp.optional && !self.is_compatible(&prop_exp.ty, &prop_act.ty) {
                                return false;
                            }
                        }
                        None => {
                            if !prop_exp.optional {
                                return false;
                            }
                        }
                    }
                }
                true
            }
            (Type::Named(name), _) => {
                if let Some(iface) = self.interfaces.get(name) {
                    self.is_compatible(&Type::Object(iface.properties.clone()), actual)
                } else if let Some(alias) = self.type_aliases.get(name) {
                    self.is_compatible(&alias.ty, actual)
                } else {
                    false
                }
            }
            (_, Type::Named(name)) => {
                if let Some(iface) = self.interfaces.get(name) {
                    self.is_compatible(expected, &Type::Object(iface.properties.clone()))
                } else if let Some(alias) = self.type_aliases.get(name) {
                    self.is_compatible(expected, &alias.ty)
                } else {
                    false
                }
            }
            (Type::Function { params: p1, return_type: r1 }, Type::Function { params: p2, return_type: r2 }) => {
                p1.len() == p2.len()
                    && p1.iter().zip(p2.iter()).all(|(a, b)| self.is_compatible(a, b))
                    && self.is_compatible(r1, r2)
            }
            _ => false,
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
