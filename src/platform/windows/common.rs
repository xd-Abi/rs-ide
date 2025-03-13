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
        use std::ffi::CString;
        PCSTR(CString::new($s).expect("Failed to convert to c-string").as_ptr() as _)
    }};
}

#[macro_export]
macro_rules! loword {
    ($lparam:expr) => {
        ($lparam.0 & 0xFFFF) as u16
    };
}

#[macro_export]
macro_rules! hiword {
    ($lparam:expr) => {
        (($lparam.0 >> 16) & 0xFFFF) as u16
    };
}

#[macro_export]
macro_rules! get_window_mut {
    ($handle:expr, $msg:expr, $w_param:expr, $l_param:expr) => {{
        use tracing::error;

        let window_ptr = GetWindowLongPtrW($handle, GWLP_USERDATA) as *mut Window;
        if window_ptr.is_null() {
            error!("Window pointer is null");
            return DefWindowProcA($handle, $msg, $w_param, $l_param);
        }

        &mut *window_ptr
    }};
}

pub fn get_instance_handle() -> HINSTANCE {
    unsafe {
        GetModuleHandleA(None)
            .expect("Failed to get module handle.")
            .into()
    }
}