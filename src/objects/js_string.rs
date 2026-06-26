#[derive(Debug, Clone, PartialEq)]
pub struct JsString {
    value: String,
}

impl JsString {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
    
    pub fn from_string(value: String) -> Self {
        Self { value }
    }
    
    pub fn as_str(&self) -> &str {
        &self.value
    }
    
    pub fn len(&self) -> usize {
        self.value.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    
    pub fn char_at(&self, index: usize) -> Option<char> {
        self.value.chars().nth(index)
    }
    
    pub fn substring(&self, start: usize, end: usize) -> JsString {
        let end = end.min(self.value.len());
        let start = start.min(end);
        
        JsString {
            value: self.value[start..end].to_string(),
        }
    }
    
    pub fn to_uppercase(&self) -> JsString {
        JsString {
            value: self.value.to_uppercase(),
        }
    }
    
    pub fn to_lowercase(&self) -> JsString {
        JsString {
            value: self.value.to_lowercase(),
        }
    }
    
    pub fn trim(&self) -> JsString {
        JsString {
            value: self.value.trim().to_string(),
        }
    }
    
    pub fn trim_start(&self) -> JsString {
        JsString {
            value: self.value.trim_start().to_string(),
        }
    }
    
    pub fn trim_end(&self) -> JsString {
        JsString {
            value: self.value.trim_end().to_string(),
        }
    }
    
    pub fn starts_with(&self, pattern: &str) -> bool {
        self.value.starts_with(pattern)
    }
    
    pub fn ends_with(&self, pattern: &str) -> bool {
        self.value.ends_with(pattern)
    }
    
    pub fn includes(&self, pattern: &str) -> bool {
        self.value.contains(pattern)
    }
    
    pub fn index_of(&self, pattern: &str) -> Option<usize> {
        self.value.find(pattern)
    }
    
    pub fn replace(&self, pattern: &str, replacement: &str) -> JsString {
        JsString {
            value: self.value.replace(pattern, replacement),
        }
    }
    
    pub fn split(&self, separator: &str) -> Vec<JsString> {
        self.value
            .split(separator)
            .map(|s| JsString::new(s))
            .collect()
    }
    
    pub fn repeat(&self, count: usize) -> JsString {
        JsString {
            value: self.value.repeat(count),
        }
    }
    
    pub fn pad_start(&self, target_length: usize, pad_string: &str) -> JsString {
        let padding_needed = target_length.saturating_sub(self.value.len());
        if padding_needed == 0 {
            return self.clone();
        }
        
        let pad_chars: Vec<char> = pad_string.chars().collect();
        let mut padding = String::new();
        
        for i in 0..padding_needed {
            padding.push(pad_chars[i % pad_chars.len()]);
        }
        
        JsString {
            value: format!("{}{}", padding, self.value),
        }
    }
    
    pub fn pad_end(&self, target_length: usize, pad_string: &str) -> JsString {
        let padding_needed = target_length.saturating_sub(self.value.len());
        if padding_needed == 0 {
            return self.clone();
        }
        
        let pad_chars: Vec<char> = pad_string.chars().collect();
        let mut padding = String::new();
        
        for i in 0..padding_needed {
            padding.push(pad_chars[i % pad_chars.len()]);
        }
        
        JsString {
            value: format!("{}{}", self.value, padding),
        }
    }
    
    pub fn concat(&self, other: &JsString) -> JsString {
        JsString {
            value: format!("{}{}", self.value, other.value),
        }
    }
    
    pub fn to_number(&self) -> Option<f64> {
        self.value.parse().ok()
    }
    
    pub fn to_int(&self, radix: u32) -> Option<i64> {
        i64::from_str_radix(&self.value, radix).ok()
    }
}

impl std::fmt::Display for JsString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for JsString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for JsString {
    fn from(s: String) -> Self {
        Self::from_string(s)
    }
}
