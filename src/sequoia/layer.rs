use glium::{
    glutin::{self, event_loop::ControlFlow},
    Display, Frame,
};

use super::input::Input;

pub trait Layer {
    fn on_attach(&mut self) {}
    fn on_detach(&mut self) {}
    fn on_update(&mut self, _input: &Input) {}
    fn on_event(&mut self, _event: &mut Option<super::event::Event>) {}
    fn on_egui_render(
        &mut self,
        _input: &Input,
        _display: &Display,
        _target: &mut Frame,
        _event: &glutin::event::Event<()>,
        _control_flow: &mut ControlFlow,
        _egui_glium: &mut egui_glium::EguiGlium,
    ) {}
}

pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
    first_overlay_index: usize,
}

fn cmp<T>(a1: &T, a2: &T) -> bool {
    a1 as *const _ == a2 as *const _
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            first_overlay_index: 0,
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.insert(self.first_overlay_index, layer);
        self.first_overlay_index += 1;
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.push(overlay);
    }

    pub fn pop_layer(&mut self, layer: Box<dyn Layer>) {
        for i in 0..self.first_overlay_index {
            if cmp(&layer, &self.layers[i]) {
                self.layers.remove(i);
                self.first_overlay_index -= 1;
                return;
            }
        }
    }

    pub fn pop_overlay(&mut self, overlay: Box<dyn Layer>) {
        for i in self.first_overlay_index..self.layers.len() {
            if cmp(&overlay, &self.layers[i]) {
                self.layers.remove(i);
                return;
            }
        }
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Box<dyn Layer>> {
        self.layers.iter_mut()
    }
}
