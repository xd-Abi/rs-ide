use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

#[macro_export]
macro_rules! static_pcstr {
    ($s:expr) => {{
        use windows::core::PCSTR;
        PCSTR($s.as_ptr() as _)
    }};
}

#[macro_export]
macro_rules! pcstr {
    ($s:expr) => {{
        use windows::core::PCSTR;
        use std::ffi::CString;;
        PCSTR(CString::new($s).expect("Failed to convert to c-string").as_ptr() as _)
    }};
}

pub fn get_instance_handle() -> HINSTANCE {
    unsafe {
        GetModuleHandleA(None)
            .expect("Failed to get module handle.")
            .into()
    }
}
