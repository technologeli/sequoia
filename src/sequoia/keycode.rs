use glium::glutin::event::{VirtualKeyCode, VirtualKeyCode::*};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum KeyCode {
    Space = 32,
    Apostrophe = 39, /* ' */
    Comma = 44,      /* , */
    Minus = 45,      /* - */
    Period = 46,     /* . */
    Slash = 47,      /* / */

    D0 = 48, /* 0 */
    D1 = 49, /* 1 */
    D2 = 50, /* 2 */
    D3 = 51, /* 3 */
    D4 = 52, /* 4 */
    D5 = 53, /* 5 */
    D6 = 54, /* 6 */
    D7 = 55, /* 7 */
    D8 = 56, /* 8 */
    D9 = 57, /* 9 */

    Semicolon = 59, /* ; */
    Equal = 61,     /* = */

    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,

    LeftBracket = 91,  /* [ */
    Backslash = 92,    /* \ */
    RightBracket = 93, /* ] */
    GraveAccent = 96,  /* ` */

    World1 = 161, /* non-US #1 */
    World2 = 162, /* non-US #2 */

    /* Function keys */
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    F13 = 302,
    F14 = 303,
    F15 = 304,
    F16 = 305,
    F17 = 306,
    F18 = 307,
    F19 = 308,
    F20 = 309,
    F21 = 310,
    F22 = 311,
    F23 = 312,
    F24 = 313,
    F25 = 314,

    /* Keypad */
    KP0 = 320,
    KP1 = 321,
    KP2 = 322,
    KP3 = 323,
    KP4 = 324,
    KP5 = 325,
    KP6 = 326,
    KP7 = 327,
    KP8 = 328,
    KP9 = 329,
    KPDecimal = 330,
    KPDivide = 331,
    KPMultiply = 332,
    KPSubtract = 333,
    KPAdd = 334,
    KPEnter = 335,
    KPEqual = 336,

    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    Menu = 348,
}

impl KeyCode {
    pub fn convert(from: VirtualKeyCode) -> KeyCode {
        match from {
            Space => KeyCode::Space,
            Apostrophe => KeyCode::Apostrophe, /* ' */
            Comma => KeyCode::Comma,           /* , */
            Minus => KeyCode::Minus,           /* - */
            Period => KeyCode::Period,         /* . */
            Slash => KeyCode::Slash,           /* / */
            Key0 => KeyCode::D0,               /* 0 */
            Key1 => KeyCode::D1,               /* 1 */
            Key2 => KeyCode::D2,               /* 2 */
            Key3 => KeyCode::D3,               /* 3 */
            Key4 => KeyCode::D4,               /* 4 */
            Key5 => KeyCode::D5,               /* 5 */
            Key6 => KeyCode::D6,               /* 6 */
            Key7 => KeyCode::D7,               /* 7 */
            Key8 => KeyCode::D8,               /* 8 */
            Key9 => KeyCode::D9,               /* 9 */
            Semicolon => KeyCode::Semicolon,   /* ; */
            Equals => KeyCode::Equal,          /* = */
            A => KeyCode::A,
            B => KeyCode::B,
            C => KeyCode::C,
            D => KeyCode::D,
            E => KeyCode::E,
            F => KeyCode::F,
            G => KeyCode::G,
            H => KeyCode::H,
            I => KeyCode::I,
            J => KeyCode::J,
            K => KeyCode::K,
            L => KeyCode::L,
            M => KeyCode::M,
            N => KeyCode::N,
            O => KeyCode::O,
            P => KeyCode::P,
            Q => KeyCode::Q,
            R => KeyCode::R,
            S => KeyCode::S,
            T => KeyCode::T,
            U => KeyCode::U,
            V => KeyCode::V,
            W => KeyCode::W,
            X => KeyCode::X,
            Y => KeyCode::Y,
            Z => KeyCode::Z,
            LBracket => KeyCode::LeftBracket, /* [ */
            Backslash => KeyCode::Backslash,  /* \ */
            RBracket => KeyCode::RightBracket, /* ] */
            Grave => KeyCode::GraveAccent,    /* ` */
            Escape => KeyCode::Escape,
            Return => KeyCode::Enter,
            Tab => KeyCode::Tab,
            Back => KeyCode::Backspace,
            Insert => KeyCode::Insert,
            Delete => KeyCode::Delete,
            Right => KeyCode::Right,
            Left => KeyCode::Left,
            Down => KeyCode::Down,
            Up => KeyCode::Up,
            PageUp => KeyCode::PageUp,
            PageDown => KeyCode::PageDown,
            Home => KeyCode::Home,
            End => KeyCode::End,
            Numlock => KeyCode::NumLock,
            Pause => KeyCode::Pause,
            F1 => KeyCode::F1,
            F2 => KeyCode::F2,
            F3 => KeyCode::F3,
            F4 => KeyCode::F4,
            F5 => KeyCode::F5,
            F6 => KeyCode::F6,
            F7 => KeyCode::F7,
            F8 => KeyCode::F8,
            F9 => KeyCode::F9,
            F10 => KeyCode::F10,
            F11 => KeyCode::F11,
            F12 => KeyCode::F12,
            F13 => KeyCode::F13,
            F14 => KeyCode::F14,
            F15 => KeyCode::F15,
            F16 => KeyCode::F16,
            F17 => KeyCode::F17,
            F18 => KeyCode::F18,
            F19 => KeyCode::F19,
            F20 => KeyCode::F20,
            F21 => KeyCode::F21,
            F22 => KeyCode::F22,
            F23 => KeyCode::F23,
            F24 => KeyCode::F24,
            Numpad0 => KeyCode::KP0,
            Numpad1 => KeyCode::KP1,
            Numpad2 => KeyCode::KP2,
            Numpad3 => KeyCode::KP3,
            Numpad4 => KeyCode::KP4,
            Numpad5 => KeyCode::KP5,
            Numpad6 => KeyCode::KP6,
            Numpad7 => KeyCode::KP7,
            Numpad8 => KeyCode::KP8,
            Numpad9 => KeyCode::KP9,
            NumpadDecimal => KeyCode::KPDecimal,
            NumpadDivide => KeyCode::KPDivide,
            NumpadMultiply => KeyCode::KPMultiply,
            NumpadSubtract => KeyCode::KPSubtract,
            NumpadAdd => KeyCode::KPAdd,
            NumpadEnter => KeyCode::KPEnter,
            NumpadEquals => KeyCode::KPEqual,
            LShift => KeyCode::LeftShift,
            LControl => KeyCode::LeftControl,
            LAlt => KeyCode::LeftAlt,
            RShift => KeyCode::RightShift,
            RControl => KeyCode::RightControl,
            RAlt => KeyCode::RightAlt,
            // dont care
            _ => KeyCode::KPDivide
        }
    }
}
