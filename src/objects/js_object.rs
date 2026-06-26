use std::collections::HashMap;
use crate::objects::Value;

#[derive(Debug, Clone)]
pub struct JsObject {
    pub properties: HashMap<String, Value>,
    pub prototype: Option<usize>,
}

impl JsObject {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            prototype: None,
        }
    }
    
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }
    
    pub fn set(&mut self, key: &str, value: Value) {
        self.properties.insert(key.to_string(), value);
    }
    
    pub fn has(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }
    
    pub fn delete(&mut self, key: &str) -> bool {
        self.properties.remove(key).is_some()
    }
    
    pub fn keys(&self) -> Vec<&str> {
        self.properties.keys().map(|s| s.as_str()).collect()
    }
    
    pub fn values(&self) -> Vec<&Value> {
        self.properties.values().collect()
    }
    
    pub fn entries(&self) -> Vec<(&str, &Value)> {
        self.properties.iter().map(|(k, v)| (k.as_str(), v)).collect()
    }
}

impl Default for JsObject {
    fn default() -> Self {
        Self::new()
    }
}
