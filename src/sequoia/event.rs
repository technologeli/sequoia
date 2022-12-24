use super::{keycode::KeyCode, mousecode::MouseCode};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    WindowClose,
    WindowResize { width: u32, height: u32 },
    KeyPress { key: KeyCode },
    KeyRelease { key: KeyCode },
    MouseButtonPress { mouse_button: MouseCode },
    MouseButtonRelease { mouse_button: MouseCode },
    MouseScroll { x: f64, y: f64 },
    MouseMove { x: f64, y: f64 }
}
