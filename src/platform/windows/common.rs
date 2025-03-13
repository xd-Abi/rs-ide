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
        use std::ffi::CString;
        use windows::core::PCSTR;
        PCSTR(
            CString::new($s)
                .expect("Failed to convert to c-string")
                .as_ptr() as _,
        )
    }};
}

#[macro_export]
macro_rules! loword {
    ($l_param:expr) => {
        ($l_param.0 & 0xFFFF) as u16
    };
}

#[macro_export]
macro_rules! hiword {
    ($l_param:expr) => {
        (($l_param.0 >> 16) & 0xFFFF) as u16
    };
}

#[macro_export]
macro_rules! get_x_lparam {
    ($l_param:expr) => {
        ($l_param.0 & 0xFFFF) as i16
    };
}

#[macro_export]
macro_rules! get_y_lparam {
    ($l_param:expr) => {
        (($l_param.0 >> 16) & 0xFFFF) as i16
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
