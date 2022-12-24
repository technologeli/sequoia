use std::collections::HashSet;

use super::{keycode::KeyCode, mousecode::MouseCode};

pub struct Input {
    pub keys_pressed: HashSet<KeyCode>,
    pub mouse_pressed: HashSet<MouseCode>,
    pub mouse_location: (f64, f64),
}

