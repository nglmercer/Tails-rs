use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

/// A safe wrapper around a raw pointer with lifetime tracking
pub struct SafePtr<'a, T> {
    ptr: *mut T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> SafePtr<'a, T> {
    /// Create a new SafePtr from a raw pointer
    /// 
    /// # Safety
    /// The caller must ensure:
    /// - The pointer is valid for the lifetime 'a
    /// - The pointer is properly aligned
    /// - No other pointer/reference aliases this memory
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Get a reference to the underlying data
    /// 
    /// # Safety
    /// The pointer must be valid and properly aligned
    pub unsafe fn as_ref(&self) -> &'a T {
        &*self.ptr
    }

    /// Get a mutable reference to the underlying data
    /// 
    /// # Safety
    /// The pointer must be valid, properly aligned, and no other references exist
    pub unsafe fn as_mut(&mut self) -> &'a mut T {
        &mut *self.ptr
    }

    /// Check if the pointer is null
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

/// A safe wrapper around a C string pointer
pub struct SafeCStr<'a> {
    ptr: *const c_char,
    _marker: PhantomData<&'a CStr>,
}

impl<'a> SafeCStr<'a> {
    /// Create a new SafeCStr from a raw pointer
    /// 
    /// # Safety
    /// The pointer must point to a valid null-terminated C string
    pub unsafe fn new(ptr: *const c_char) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Convert to a Rust string slice
    /// 
    /// Returns None if the pointer is null or the string is not valid UTF-8
    pub fn to_str(&self) -> Option<&'a str> {
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

/// A safe wrapper around a raw slice
pub struct SafeSlice<'a, T> {
    ptr: *const T,
    len: usize,
    _marker: PhantomData<&'a [T]>,
}

impl<'a, T> SafeSlice<'a, T> {
    /// Create a new SafeSlice from a raw pointer and length
    /// 
    /// # Safety
    /// The pointer must be valid for `len` elements
    pub unsafe fn new(ptr: *const T, len: usize) -> Self {
        Self {
            ptr,
            len,
            _marker: PhantomData,
        }
    }

    /// Get a slice of the data
    /// 
    /// # Safety
    /// The pointer must be valid for `len` elements
    pub unsafe fn as_slice(&self) -> &'a [T] {
        std::slice::from_raw_parts(self.ptr, self.len)
    }

    /// Get the length of the slice
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the slice is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_safe_ptr_null() {
        let ptr = unsafe { SafePtr::<i32>::new(std::ptr::null_mut()) };
        assert!(ptr.is_null());
    }

    #[test]
    fn test_safe_cstr_null() {
        let cstr = unsafe { SafeCStr::new(std::ptr::null()) };
        assert!(cstr.is_null());
        assert!(cstr.to_str().is_none());
    }

    #[test]
    fn test_safe_cstr_valid() {
        let c_string = CString::new("hello").unwrap();
        let cstr = unsafe { SafeCStr::new(c_string.as_ptr()) };
        assert!(!cstr.is_null());
        assert_eq!(cstr.to_str(), Some("hello"));
    }

    #[test]
    fn test_safe_slice_empty() {
        let slice = unsafe { SafeSlice::<i32>::new(std::ptr::null(), 0) };
        assert!(slice.is_empty());
        assert_eq!(slice.len(), 0);
    }
}