use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// A safe wrapper for FFI string operations
pub struct SafeFFIString {
    ptr: *mut c_char,
    owned: bool,
}

impl SafeFFIString {
    /// Create a new safe string from a C string pointer
    ///
    /// # Safety
    /// The pointer must be valid and null-terminated
    pub unsafe fn from_raw(ptr: *const c_char) -> Self {
        Self {
            ptr: ptr as *mut c_char,
            owned: false,
        }
    }

    /// Create an owned copy of the string
    pub fn to_owned(&self) -> Option<String> {
        if self.ptr.is_null() {
            return None;
        }
        unsafe { CStr::from_ptr(self.ptr) }
            .to_str()
            .ok()
            .map(|s| s.to_string())
    }

    /// Get a reference to the string as a Rust str
    pub fn as_str(&self) -> Option<&str> {
        if self.ptr.is_null() {
            return None;
        }
        unsafe { CStr::from_ptr(self.ptr) }.to_str().ok()
    }

    /// Check if the pointer is null
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl Drop for SafeFFIString {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe {
                let _ = CString::from_raw(self.ptr);
            }
        }
    }
}

/// A buffer for safe string allocation in FFI
pub struct FFIStringBuffer {
    strings: Vec<CString>,
}

impl FFIStringBuffer {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
        }
    }

    /// Allocate a new string and return its pointer
    pub fn alloc(&mut self, s: &str) -> *const c_char {
        match CString::new(s) {
            Ok(c_string) => {
                let ptr = c_string.as_ptr();
                self.strings.push(c_string);
                ptr
            }
            Err(_) => ptr::null(),
        }
    }

    /// Get the number of allocated strings
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
}

impl Default for FFIStringBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_ffi_string_null() {
        let s = unsafe { SafeFFIString::from_raw(std::ptr::null()) };
        assert!(s.is_null());
        assert!(s.as_str().is_none());
        assert!(s.to_owned().is_none());
    }

    #[test]
    fn test_safe_ffi_string_valid() {
        let c_string = CString::new("hello").unwrap();
        let s = unsafe { SafeFFIString::from_raw(c_string.as_ptr()) };
        assert!(!s.is_null());
        assert_eq!(s.as_str(), Some("hello"));
        assert_eq!(s.to_owned(), Some("hello".to_string()));
    }

    #[test]
    fn test_ffi_string_buffer() {
        let mut buffer = FFIStringBuffer::new();
        assert!(buffer.is_empty());

        let ptr1 = buffer.alloc("hello");
        assert!(!ptr1.is_null());
        assert_eq!(buffer.len(), 1);

        let ptr2 = buffer.alloc("world");
        assert!(!ptr2.is_null());
        assert_eq!(buffer.len(), 2);

        unsafe {
            assert_eq!(CStr::from_ptr(ptr1).to_str().unwrap(), "hello");
            assert_eq!(CStr::from_ptr(ptr2).to_str().unwrap(), "world");
        }
    }
}
