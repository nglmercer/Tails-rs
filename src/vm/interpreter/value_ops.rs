use super::Interpreter;
use crate::errors::{Error, Result};
use crate::objects::Value;

impl Interpreter {
    pub(super) fn add(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(Error::TypeError(format!(
                "Cannot add {} and {}",
                self.value_to_string(&left),
                self.value_to_string(&right)
            ))),
        }
    }

    pub(super) fn sub(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot subtract {} from {}",
                self.value_to_string(&right),
                self.value_to_string(&left)
            ))),
        }
    }

    pub(super) fn mul(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * *b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot multiply {} and {}",
                self.value_to_string(&left),
                self.value_to_string(&right)
            ))),
        }
    }

    pub(super) fn div(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Float(*a as f64 / *b as f64))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Float(a / b))
            }
            (Value::Integer(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Float(*a as f64 / b))
            }
            (Value::Float(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Float(a / *b as f64))
            }
            _ => Err(Error::TypeError(format!(
                "Cannot divide {} by {}",
                self.value_to_string(&left),
                self.value_to_string(&right)
            ))),
        }
    }

    pub(super) fn modulo(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Integer(a % b))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Float(a % b))
            }
            _ => Err(Error::TypeError(format!(
                "Cannot apply modulo to {} and {}",
                self.value_to_string(&left),
                self.value_to_string(&right)
            ))),
        }
    }

    pub(super) fn power(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Float((*a as f64).powf(*b as f64))),
            _ => Err(Error::TypeError(format!(
                "Cannot raise {} to the power of {}",
                self.value_to_string(&left),
                self.value_to_string(&right)
            ))),
        }
    }

    pub(super) fn negate(&self, value: Value) -> Result<Value> {
        match value {
            Value::Integer(n) => Ok(Value::Integer(-n)),
            Value::Float(n) => Ok(Value::Float(-n)),
            _ => Err(Error::TypeError(format!(
                "Cannot negate {}",
                self.value_to_string(&value)
            ))),
        }
    }

    pub(crate) fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Undefined => false,
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Integer(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::BigInt(n) => *n != 0,
            _ => true,
        }
    }

    pub(super) fn is_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Null, Value::Null) => true,
            (Value::Null, Value::Undefined) => true,
            (Value::Undefined, Value::Null) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b && !a.is_nan() && !b.is_nan(),
            (Value::Integer(a), Value::Float(b)) => *a as f64 == *b && !b.is_nan(),
            (Value::Float(a), Value::Integer(b)) => *a == *b as f64 && !a.is_nan(),
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }

    pub(super) fn less_than(&self, left: &Value, right: &Value) -> Result<bool> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a < b),
            (Value::Float(a), Value::Float(b)) => Ok(a < b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f64) < *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a < (*b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot compare {} and {}",
                self.value_to_string(left),
                self.value_to_string(right)
            ))),
        }
    }

    pub(super) fn greater_than(&self, left: &Value, right: &Value) -> Result<bool> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a > b),
            (Value::Float(a), Value::Float(b)) => Ok(a > b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f64) > *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a > (*b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot compare {} and {}",
                self.value_to_string(left),
                self.value_to_string(right)
            ))),
        }
    }

    pub(super) fn less_than_or_equal(&self, left: &Value, right: &Value) -> Result<bool> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a <= b),
            (Value::Float(a), Value::Float(b)) => Ok(a <= b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f64) <= *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a <= (*b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot compare {} and {}",
                self.value_to_string(left),
                self.value_to_string(right)
            ))),
        }
    }

    pub(super) fn greater_than_or_equal(&self, left: &Value, right: &Value) -> Result<bool> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a >= b),
            (Value::Float(a), Value::Float(b)) => Ok(a >= b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f64) >= *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a >= (*b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot compare {} and {}",
                self.value_to_string(left),
                self.value_to_string(right)
            ))),
        }
    }

    pub(super) fn to_number(&self, value: &Value) -> Result<f64> {
        match value {
            Value::Integer(n) => Ok(*n as f64),
            Value::Float(n) => Ok(*n),
            Value::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Value::Null => Ok(0.0),
            Value::Undefined => Ok(f64::NAN),
            Value::String(s) => {
                if s.is_empty() {
                    Ok(0.0)
                } else {
                    Ok(s.parse::<f64>().unwrap_or(f64::NAN))
                }
            }
            _ => Ok(f64::NAN),
        }
    }

    pub(super) fn value_to_string_raw(&self, value: &Value) -> String {
        match value {
            Value::Undefined => "undefined".to_string(),
            Value::Null => "null".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Float(n) => {
                if *n == (*n as i64) as f64 {
                    (*n as i64).to_string()
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::BigInt(n) => format!("{}n", n),
            Value::Function(_) => "[Function]".to_string(),
            Value::NativeFunction(_) => "[Native Function]".to_string(),
            Value::Object(_) => "[Object]".to_string(),
            Value::Array(_) => "[Array]".to_string(),
            Value::Promise(_) => "[Promise]".to_string(),
            Value::Proxy(_) => "[Proxy]".to_string(),
            Value::Generator(_) => "[Generator]".to_string(),
        }
    }

    pub(super) fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::Undefined => "undefined".to_string(),
            Value::Null => "null".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::String(s) => format!("\"{}\"", s),
            Value::BigInt(n) => format!("{}n", n),
            Value::Function(_) => "[Function]".to_string(),
            Value::NativeFunction(_) => "[Native Function]".to_string(),
            Value::Object(_) => "[Object]".to_string(),
            Value::Array(_) => "[Array]".to_string(),
            Value::Promise(_) => "[Promise]".to_string(),
            Value::Proxy(_) => "[Proxy]".to_string(),
            Value::Generator(_) => "[Generator]".to_string(),
        }
    }
}
