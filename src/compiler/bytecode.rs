use crate::compiler::parser::{AstNode, Statement, Expression, BinaryOperator};
use crate::compiler::{CompiledModule, Instruction};
use crate::errors::Result;
use crate::objects::Value;

pub fn generate(ast: &AstNode) -> Result<CompiledModule> {
    let mut generator = CodeGenerator::new();
    generator.generate(ast)
}

struct CodeGenerator {
    constants: Vec<Value>,
    instructions: Vec<Instruction>,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            constants: Vec::new(),
            instructions: Vec::new(),
        }
    }
    
    fn generate(&mut self, ast: &AstNode) -> Result<CompiledModule> {
        match ast {
            AstNode::Program(statements) => {
                for stmt in statements {
                    self.generate_statement(stmt)?;
                }
                self.instructions.push(Instruction::LoadUndefined);
                self.instructions.push(Instruction::Return);
            }
            _ => return Err(crate::errors::Error::InternalError("Invalid AST node".into())),
        }
        
        Ok(CompiledModule {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        })
    }
    
    fn generate_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Expression(expr) => {
                self.generate_expression(expr)?;
                self.instructions.push(Instruction::Pop);
                Ok(())
            }
            Statement::VariableDeclaration { declarations, .. } => {
                for decl in declarations {
                    if let Some(init) = &decl.init {
                        self.generate_expression(init)?;
                    } else {
                        self.instructions.push(Instruction::LoadUndefined);
                    }
                    self.instructions.push(Instruction::StoreGlobal(decl.id.clone()));
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
                let jump_to_else = self.instructions.len();
                self.instructions.push(Instruction::LoadFalse);
                self.instructions.push(Instruction::Pop);
                
                self.generate_statement(consequent)?;
                
                if let Some(alt) = alternate {
                    let jump_to_end = self.instructions.len();
                    self.instructions.push(Instruction::LoadFalse);
                    self.instructions.push(Instruction::Pop);
                    
                    self.patch_jump(jump_to_else, self.instructions.len());
                    self.generate_statement(alt)?;
                    self.patch_jump(jump_to_end, self.instructions.len());
                } else {
                    self.patch_jump(jump_to_else, self.instructions.len());
                }
                
                Ok(())
            }
            Statement::WhileStatement { condition, body } => {
                let loop_start = self.instructions.len();
                self.generate_expression(condition)?;
                
                let jump_to_end = self.instructions.len();
                self.instructions.push(Instruction::LoadFalse);
                self.instructions.push(Instruction::Pop);
                
                self.generate_statement(body)?;
                self.instructions.push(Instruction::LoadFalse);
                self.instructions.push(Instruction::Pop);
                
                self.patch_jump(jump_to_end, self.instructions.len());
                Ok(())
            }
            Statement::BlockStatement(stmts) => {
                for stmt in stmts {
                    self.generate_statement(stmt)?;
                }
                Ok(())
            }
            Statement::FunctionDeclaration { .. } => {
                // TODO: Implement function compilation
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
                self.instructions.push(Instruction::LoadGlobal(name.clone()));
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
                    _ => {}
                }
                Ok(())
            }
            Expression::Assignment { target, value } => {
                self.generate_expression(value)?;
                if let Expression::Identifier(name) = target.as_ref() {
                    self.instructions.push(Instruction::StoreGlobal(name.clone()));
                } else {
                    return Err(crate::errors::Error::RuntimeError("Invalid assignment target".into()));
                }
                Ok(())
            }
            Expression::Call { .. } => {
                // TODO: Implement function calls
                Ok(())
            }
            Expression::Member { .. } => {
                // TODO: Implement member access
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
            _ => {}
        }
        Ok(())
    }
    
    fn add_constant(&mut self, value: Value) -> u32 {
        let idx = self.constants.len() as u32;
        self.constants.push(value);
        idx
    }
    
    fn patch_jump(&mut self, offset: usize, target: usize) {
        // TODO: Implement jump patching
    }
}
