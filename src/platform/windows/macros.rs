#[macro_export]
macro_rules! pcstr {
    ($s:expr) => {{
        use std::ffi::CString;
        use windows::core::PCSTR;
        PCSTR(CString::new($s).expect("Failed to convert to c-string").as_ptr() as _)
    }};
}