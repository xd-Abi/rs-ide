use crate::events::Key;
use tracing::{info, warn};
use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub fn vk_to_key(vk: VIRTUAL_KEY) -> Option<Key> {
    match vk {
        VK_A => Some(Key::A),
        VK_B => Some(Key::B),
        VK_C => Some(Key::C),
        VK_D => Some(Key::D),
        VK_E => Some(Key::E),
        VK_F => Some(Key::F),
        VK_G => Some(Key::G),
        VK_H => Some(Key::H),
        VK_I => Some(Key::I),
        VK_J => Some(Key::J),
        VK_K => Some(Key::K),
        VK_L => Some(Key::L),
        VK_M => Some(Key::M),
        VK_N => Some(Key::N),
        VK_O => Some(Key::O),
        VK_P => Some(Key::P),
        VK_Q => Some(Key::Q),
        VK_R => Some(Key::R),
        VK_S => Some(Key::S),
        VK_T => Some(Key::T),
        VK_U => Some(Key::U),
        VK_V => Some(Key::V),
        VK_W => Some(Key::W),
        VK_X => Some(Key::X),
        VK_Y => Some(Key::Y),
        VK_Z => Some(Key::Z),

        // Map Number Keys 0-9
        VK_0 => Some(Key::Num0),
        VK_1 => Some(Key::Num1),
        VK_2 => Some(Key::Num2),
        VK_3 => Some(Key::Num3),
        VK_4 => Some(Key::Num4),
        VK_5 => Some(Key::Num5),
        VK_6 => Some(Key::Num6),
        VK_7 => Some(Key::Num7),
        VK_8 => Some(Key::Num8),
        VK_9 => Some(Key::Num9),

        // Special Keys
        VK_ESCAPE => Some(Key::Escape),
        VK_TAB => Some(Key::Tab),
        VK_CAPITAL => Some(Key::CapsLock),
        VK_SHIFT => Some(Key::Shift),
        VK_CONTROL => Some(Key::Ctrl),
        VK_LWIN => Some(Key::LeftSuper),
        VK_RWIN => Some(Key::RightSuper),
        VK_SPACE => Some(Key::Space),
        VK_MENU => Some(Key::Alt), // VK_MENU is used for Alt
        VK_RETURN => Some(Key::Enter),
        VK_BACK => Some(Key::Backspace),

        // Function Keys (F1 - F12)
        VK_F1 => Some(Key::F1),
        VK_F2 => Some(Key::F2),
        VK_F3 => Some(Key::F3),
        VK_F4 => Some(Key::F4),
        VK_F5 => Some(Key::F5),
        VK_F6 => Some(Key::F6),
        VK_F7 => Some(Key::F7),
        VK_F8 => Some(Key::F8),
        VK_F9 => Some(Key::F9),
        VK_F10 => Some(Key::F10),
        VK_F11 => Some(Key::F11),
        VK_F12 => Some(Key::F12),

        // Arrow Keys
        VK_LEFT => Some(Key::ArrowLeft),
        VK_RIGHT => Some(Key::ArrowRight),
        VK_UP => Some(Key::ArrowUp),
        VK_DOWN => Some(Key::ArrowDown),
        _ => {
            warn!("Unknown virtual key: {:?}", vk);
            None
        }
    }
}
