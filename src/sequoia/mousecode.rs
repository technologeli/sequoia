use glium::glutin::event::MouseButton;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MouseCode {
    Button0 = 0,
    Button1 = 1,
    Button2 = 2,
    Button3 = 3,
    Button4 = 4,
    Button5 = 5,
    Button6 = 6,
    Button7 = 7,
    // ButtonLast             = Button7,
    // ButtonLeft             = Button0,
    // ButtonRight            = Button1,
    // ButtonMiddle           = Button2
}

impl MouseCode {
    pub fn convert(from: MouseButton) -> MouseCode {
        match from {
            MouseButton::Left => MouseCode::Button0,
            MouseButton::Right => MouseCode::Button1,
            MouseButton::Middle => MouseCode::Button2,
            MouseButton::Other(o) => match o {
                3 => MouseCode::Button3,
                4 => MouseCode::Button4,
                5 => MouseCode::Button5,
                6 => MouseCode::Button6,
                7 => MouseCode::Button7,
                _ => MouseCode::Button7,
            },
        }
    }
}
