use crate::compiler::parser::{AstNode, Statement, Expression, BinaryOperator};
use crate::compiler::{CompiledModule, CompiledFunction, Instruction};
use crate::errors::Result;
use crate::objects::Value;

pub fn generate(ast: &AstNode) -> Result<CompiledModule> {
    let mut generator = CodeGenerator::new();
    generator.generate(ast)
}

struct CodeGenerator {
    constants: Vec<Value>,
    instructions: Vec<Instruction>,
    functions: Vec<CompiledFunction>,
    locals: Vec<String>,
    scope_depth: usize,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            constants: Vec::new(),
            instructions: Vec::new(),
            functions: Vec::new(),
            locals: Vec::new(),
            scope_depth: 0,
        }
    }
    
    fn generate(&mut self, ast: &AstNode) -> Result<CompiledModule> {
        match ast {
            AstNode::Program(statements) => {
                for (i, stmt) in statements.iter().enumerate() {
                    let is_last = i == statements.len() - 1;
                    self.generate_statement(stmt, is_last)?;
                }
                if statements.is_empty() {
                    self.instructions.push(Instruction::LoadUndefined);
                }
            }
            _ => return Err(crate::errors::Error::InternalError("Invalid AST node".into())),
        }
        
        Ok(CompiledModule {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
            functions: self.functions.clone(),
        })
    }
    
    fn generate_statement(&mut self, stmt: &Statement, is_last: bool) -> Result<()> {
        match stmt {
            Statement::Expression(expr) => {
                self.generate_expression(expr)?;
                if !is_last {
                    self.instructions.push(Instruction::Pop);
                }
                Ok(())
            }
            Statement::VariableDeclaration { kind: _, declarations } => {
                for decl in declarations {
                    if let Some(init) = &decl.init {
                        self.generate_expression(init)?;
                    } else {
                        self.instructions.push(Instruction::LoadUndefined);
                    }
                    
                    if self.scope_depth == 0 {
                        self.instructions.push(Instruction::StoreGlobal(decl.id.clone()));
                    } else {
                        self.locals.push(decl.id.clone());
                        let slot = (self.locals.len() - 1) as u16;
                        self.instructions.push(Instruction::StoreLocal(slot));
                    }
                }
                Ok(())
            }
            Statement::ReturnStatement(value) => {
                if let Some(expr) = value {
                    self.generate_expression(expr)?;
                } else {
                    self.instructions.push(Instruction::LoadUndefined);
                }
                self.instructions.push(Instruction::Return);
                Ok(())
            }
            Statement::IfStatement { condition, consequent, alternate } => {
                self.generate_expression(condition)?;
                
                let jump_if_not = self.instructions.len();
                self.instructions.push(Instruction::JumpIfNot(0));
                
                self.generate_statement(consequent, false)?;
                
                if let Some(alt) = alternate {
                    let jump_to_end = self.instructions.len();
                    self.instructions.push(Instruction::Jump(0));
                    
                    self.patch_jump(jump_if_not, self.instructions.len());
                    self.generate_statement(alt, false)?;
                    
                    self.patch_jump(jump_to_end, self.instructions.len());
                } else {
                    self.patch_jump(jump_if_not, self.instructions.len());
                }
                
                Ok(())
            }
            Statement::WhileStatement { condition, body } => {
                let loop_start = self.instructions.len() as u32;
                
                self.generate_expression(condition)?;
                
                let jump_if_not = self.instructions.len();
                self.instructions.push(Instruction::JumpIfNot(0));
                
                self.generate_statement(body, false)?;
                
                self.instructions.push(Instruction::Jump(loop_start));
                
                self.patch_jump(jump_if_not, self.instructions.len());
                
                Ok(())
            }
            Statement::BlockStatement(stmts) => {
                self.scope_depth += 1;
                let prev_locals_count = self.locals.len();
                
                for stmt in stmts {
                    self.generate_statement(stmt, false)?;
                }
                
                let locals_added = self.locals.len() - prev_locals_count;
                for _ in 0..locals_added {
                    self.locals.pop();
                    self.instructions.push(Instruction::Pop);
                }
                
                self.scope_depth -= 1;
                Ok(())
            }
            Statement::FunctionDeclaration { name, params, body, is_async: _ } => {
                let func_idx = self.functions.len() as u32;
                
                self.functions.push(CompiledFunction {
                    name: Some(name.clone()),
                    params: params.clone(),
                    bytecode_index: 0,
                    param_count: params.len(),
                });
                
                let jump_over = self.instructions.len();
                self.instructions.push(Instruction::Jump(0));
                
                let func_start = self.instructions.len();
                self.functions[func_idx as usize].bytecode_index = func_start;
                
                self.scope_depth += 1;
                let prev_locals = self.locals.len();
                
                for param in params {
                    self.locals.push(param.clone());
                }
                
                for stmt in body {
                    self.generate_statement(stmt, false)?;
                }
                
                self.instructions.push(Instruction::LoadUndefined);
                self.instructions.push(Instruction::Return);
                
                self.scope_depth -= 1;
                self.locals.truncate(prev_locals);
                
                self.patch_jump(jump_over, self.instructions.len());
                
                self.instructions.push(Instruction::MakeFunction(func_idx));
                if self.scope_depth == 0 {
                    self.instructions.push(Instruction::StoreGlobal(name.clone()));
                } else {
                    self.locals.push(name.clone());
                    let slot = (self.locals.len() - 1) as u16;
                    self.instructions.push(Instruction::StoreLocal(slot));
                }
                
                Ok(())
            }
        }
    }
    
    fn generate_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            Expression::NumberLiteral(n) => {
                let idx = self.add_constant(Value::Float(*n));
                self.instructions.push(Instruction::LoadConst(idx));
                Ok(())
            }
            Expression::StringLiteral(s) => {
                let idx = self.add_constant(Value::String(s.clone()));
                self.instructions.push(Instruction::LoadConst(idx));
                Ok(())
            }
            Expression::BooleanLiteral(b) => {
                if *b {
                    self.instructions.push(Instruction::LoadTrue);
                } else {
                    self.instructions.push(Instruction::LoadFalse);
                }
                Ok(())
            }
            Expression::NullLiteral => {
                self.instructions.push(Instruction::LoadNull);
                Ok(())
            }
            Expression::UndefinedLiteral => {
                self.instructions.push(Instruction::LoadUndefined);
                Ok(())
            }
            Expression::Identifier(name) => {
                if let Some(local_idx) = self.resolve_local(name) {
                    self.instructions.push(Instruction::LoadLocal(local_idx));
                } else {
                    self.instructions.push(Instruction::LoadGlobal(name.clone()));
                }
                Ok(())
            }
            Expression::BinaryOp { op, left, right } => {
                self.generate_expression(left)?;
                self.generate_expression(right)?;
                self.generate_binary_op(op)?;
                Ok(())
            }
            Expression::UnaryOp { op, operand } => {
                self.generate_expression(operand)?;
                match op {
                    crate::compiler::parser::UnaryOperator::Negate => {
                        self.instructions.push(Instruction::Negate);
                    }
                    crate::compiler::parser::UnaryOperator::Not => {
                        self.instructions.push(Instruction::Not);
                    }
                    crate::compiler::parser::UnaryOperator::Typeof => {
                        self.instructions.push(Instruction::TypeOf);
                    }
                    _ => {}
                }
                Ok(())
            }
            Expression::Assignment { target, value } => {
                self.generate_expression(value)?;
                if let Expression::Identifier(name) = target.as_ref() {
                    if let Some(local_idx) = self.resolve_local(name) {
                        self.instructions.push(Instruction::StoreLocal(local_idx));
                    } else {
                        self.instructions.push(Instruction::StoreGlobal(name.clone()));
                    }
                } else {
                    return Err(crate::errors::Error::RuntimeError("Invalid assignment target".into()));
                }
                Ok(())
            }
            Expression::Call { callee, args } => {
                for arg in args {
                    self.generate_expression(arg)?;
                }
                self.generate_expression(callee)?;
                self.instructions.push(Instruction::Call(args.len() as u16));
                Ok(())
            }
            Expression::Member { object, property, computed } => {
                self.generate_expression(object)?;
                if *computed {
                    self.generate_expression(property)?;
                } else if let Expression::Identifier(name) = property.as_ref() {
                    let idx = self.add_constant(Value::String(name.clone()));
                    self.instructions.push(Instruction::LoadConst(idx));
                } else {
                    self.generate_expression(property)?;
                }
                self.instructions.push(Instruction::GetProperty);
                Ok(())
            }
            Expression::FunctionExpression { name: _, params, body } => {
                let func_idx = self.functions.len() as u32;
                
                self.functions.push(CompiledFunction {
                    name: None,
                    params: params.clone(),
                    bytecode_index: 0,
                    param_count: params.len(),
                });
                
                let jump_over = self.instructions.len();
                self.instructions.push(Instruction::Jump(0));
                
                let func_start = self.instructions.len();
                self.functions[func_idx as usize].bytecode_index = func_start;
                
                self.scope_depth += 1;
                let prev_locals = self.locals.len();
                
                for param in params {
                    self.locals.push(param.clone());
                }
                
                for stmt in body {
                    self.generate_statement(stmt, false)?;
                }
                
                self.instructions.push(Instruction::LoadUndefined);
                self.instructions.push(Instruction::Return);
                
                self.scope_depth -= 1;
                self.locals.truncate(prev_locals);
                
                self.patch_jump(jump_over, self.instructions.len());
                
                self.instructions.push(Instruction::MakeFunction(func_idx));
                
                Ok(())
            }
        }
    }
    
    fn generate_binary_op(&mut self, op: &BinaryOperator) -> Result<()> {
        match op {
            BinaryOperator::Add => self.instructions.push(Instruction::Add),
            BinaryOperator::Sub => self.instructions.push(Instruction::Sub),
            BinaryOperator::Mul => self.instructions.push(Instruction::Mul),
            BinaryOperator::Div => self.instructions.push(Instruction::Div),
            BinaryOperator::Mod => self.instructions.push(Instruction::Mod),
            BinaryOperator::Eq => self.instructions.push(Instruction::Eq),
            BinaryOperator::StrictEq => self.instructions.push(Instruction::StrictEq),
            BinaryOperator::NotEqual => self.instructions.push(Instruction::NotEqual),
            BinaryOperator::StrictNotEqual => self.instructions.push(Instruction::StrictNotEqual),
            BinaryOperator::Less => self.instructions.push(Instruction::Less),
            BinaryOperator::Greater => self.instructions.push(Instruction::Greater),
            BinaryOperator::LessEqual => self.instructions.push(Instruction::LessEqual),
            BinaryOperator::GreaterEqual => self.instructions.push(Instruction::GreaterEqual),
            BinaryOperator::And => self.instructions.push(Instruction::And),
            BinaryOperator::Or => self.instructions.push(Instruction::Or),
            _ => {}
        }
        Ok(())
    }
    
    fn resolve_local(&self, name: &str) -> Option<u16> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local == name {
                return Some(i as u16);
            }
        }
        None
    }
    
    fn add_constant(&mut self, value: Value) -> u32 {
        let idx = self.constants.len() as u32;
        self.constants.push(value);
        idx
    }
    
    fn patch_jump(&mut self, offset: usize, target: usize) {
        let target_u32 = target as u32;
        match &mut self.instructions[offset] {
            Instruction::JumpIfNot(addr) => *addr = target_u32,
            Instruction::JumpIf(addr) => *addr = target_u32,
            Instruction::Jump(addr) => *addr = target_u32,
            _ => {}
        }
    }
}
