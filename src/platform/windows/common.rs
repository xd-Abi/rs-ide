use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

/// Converts a static byte string into a WinAPI-compatible PCSTR.
///
/// # Arguments
///
/// * `$s` - A static byte string (`&[u8]`) that must be null-terminated (`\0`).
///
/// # Example
///
/// ```rust
/// let class_name = static_pcstr!(b"MyWindowClass\0");
/// ```
///
/// # Notes:
/// - This macro should only be used with **static** strings.
/// - If you need a dynamically allocated string, use `pcstr!` instead.
#[macro_export]
macro_rules! static_pcstr {
    ($s:expr) => {{
        use windows::core::PCSTR;
        PCSTR($s.as_ptr() as _)
    }};
}

/// Converts a Rust string (`&str`) into a dynamically allocated PCSTR.
///
/// # Arguments
///
/// * `$s` - A Rust string (`&str`) that does **not** contain null bytes.
///
/// # Example
///
/// ```rust
/// let class_name = pcstr!("MyWindowClass");
/// ```
///
/// # Notes:
/// - This dynamically allocates memory via `CString::new()`.
/// - **Will panic if the string contains null bytes (`\0`)**.
#[macro_export]
macro_rules! pcstr {
    ($s:expr) => {{
        use std::ffi::CString;
        use windows::core::PCSTR;
        PCSTR(
            CString::new($s)
                .expect("Failed to convert to c-string")
                .as_ptr() as _,
        )
    }};
}

/// The default class name for window registration.
///
/// This name is used when registering a window class with the Windows API.
pub const WINDOW_CLASS_NAME: &[u8] = b"RustIdeWindow\0";

/// This function is needed for registering window classes in the Windows API.
///
/// # Returns
///
/// * `HINSTANCE` - A handle to the current process instance.
///
/// # Example
///
/// ```rust
/// let instance = unsafe { get_instance_handle() };
/// ```
pub unsafe fn get_instance_handle() -> HINSTANCE {
    GetModuleHandleA(None)
        .expect("Failed to get module handle.")
        .into()
}

#[cfg(test)]
mod tests {
    use windows::core::PCSTR;

    /// Tests that `static_pcstr!` correctly converts a static byte string into a `PCSTR`.
    #[test]
    fn test_static_pcstr() {
        const STATIC_STR: &[u8] = b"TestStaticClass\0";

        let pcstr_value = static_pcstr!(STATIC_STR);

        // Verify that the pointer is not null
        assert!(!pcstr_value.0.is_null(), "PCSTR pointer should not be null");

        // Verify that the pointer points to the correct value
        unsafe {
            let original_bytes =
                std::ffi::CStr::from_ptr(pcstr_value.0 as *const i8).to_bytes_with_nul();
            assert_eq!(
                original_bytes, STATIC_STR,
                "PCSTR content does not match input"
            );
        }
    }

    /// Tests that `pcstr!` correctly converts a Rust string into a dynamically allocated `PCSTR`.
    #[test]
    fn test_pcstr() {
        let input_str = "TestDynamicClass";

        let cstring = std::ffi::CString::new(input_str).expect("Failed to convert to C-string");

        // Use the macro to get the PCSTR
        let pcstr_value = PCSTR(cstring.as_ptr() as _);

        // Verify that the pointer is not null
        assert!(!pcstr_value.0.is_null(), "PCSTR pointer should not be null");

        // Verify that the pointer points to the correct string content
        unsafe {
            let original_cstring = std::ffi::CStr::from_ptr(pcstr_value.0 as *const i8);
            assert_eq!(
                original_cstring.to_str().unwrap(),
                input_str,
                "PCSTR content does not match input"
            );
        }
    }

    /// Tests that `pcstr!` panics when given a string containing a null byte (`\0`).
    #[test]
    #[should_panic(expected = "Failed to convert to c-string")]
    fn test_pcstr_panic_on_null_byte() {
        let invalid_str = "Invalid\0String"; // Contains an embedded null byte
        let _ = pcstr!(invalid_str); // This should panic
    }
}
