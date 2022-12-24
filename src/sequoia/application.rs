use std::collections::HashSet;

use glium::{glutin, Surface};

use super::{event, input, keycode, layer, mousecode};

pub struct Application {
    egui_glium: egui_glium::EguiGlium,
    layer_stack: layer::LayerStack,
    display: glium::Display,
    input: input::Input,
}

impl Application {
    pub fn new(display: glium::Display, event_loop: &glutin::event_loop::EventLoop<()>) -> Self {
        Self {
            egui_glium: egui_glium::EguiGlium::new(&display, event_loop),
            layer_stack: layer::LayerStack::new(),
            display,
            input: input::Input {
                keys_pressed: HashSet::new(),
                mouse_pressed: HashSet::new(),
                mouse_location: (0.0, 0.0),
            },
        }
    }

    pub fn on_event(&mut self, mut event: Option<event::Event>) {
        if let Some(e) = event {
            match e {
                event::Event::WindowClose => {
                    debug!("Closing window.");
                    event = None;
                }
                event::Event::WindowResize { width, height } => {
                    debug!("WindowResize {}, {}", width, height);
                    event = None;
                }
                event::Event::KeyPress { key } => {
                    // debug!("KeyPress {:?}", key);
                    self.input.keys_pressed.insert(key);
                }
                event::Event::KeyRelease { key } => {
                    // debug!("KeyRelease {:?}", key);
                    self.input.keys_pressed.remove(&key);
                }
                event::Event::MouseButtonPress { mouse_button } => {
                    // debug!("MouseButtonPress {:?}", mouse_button);
                    self.input.mouse_pressed.insert(mouse_button);
                }
                event::Event::MouseButtonRelease { mouse_button } => {
                    // debug!("MouseButtonRelease {:?}", mouse_button);
                    self.input.mouse_pressed.remove(&mouse_button);
                }
                // event::Event::MouseScroll { x, y } => debug!("MouseScroll {}, {}", x, y),
                event::Event::MouseMove { x, y } => {
                    // debug!("MouseMove {}, {}", x, y);
                    self.input.mouse_location = (x, y)
                }
                _ => {}
            }
        }

        for layer in self.layer_stack.iter_mut().rev() {
            match event {
                None => break,
                Some(_) => layer.on_event(&mut event),
            }
        }
    }

    pub fn push_layer(&mut self, mut layer: Box<dyn layer::Layer>) {
        layer.on_attach();
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, mut overlay: Box<dyn layer::Layer>) {
        overlay.on_attach();
        self.layer_stack.push_overlay(overlay);
    }

    pub fn pop_layer(&mut self, mut layer: Box<dyn layer::Layer>) {
        layer.on_detach();
        self.layer_stack.pop_layer(layer);
    }

    pub fn pop_overlay(&mut self, mut overlay: Box<dyn layer::Layer>) {
        overlay.on_detach();
        self.layer_stack.pop_overlay(overlay);
    }

    fn handle_event(
        &mut self,
        ev: glutin::event::Event<()>,
        control_flow: &mut glutin::event_loop::ControlFlow,
    ) {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    self.on_event(Some(event::Event::WindowClose));
                }
                glutin::event::WindowEvent::Resized(size) => {
                    self.on_event(Some(event::Event::WindowResize {
                        width: size.width,
                        height: size.height,
                    }));
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                    glutin::event::ElementState::Pressed => {
                        self.on_event(Some(event::Event::KeyPress {
                            key: keycode::KeyCode::convert(
                                input
                                    .virtual_keycode
                                    .unwrap_or(glutin::event::VirtualKeyCode::NumpadDivide),
                            ),
                        }));
                    }
                    glutin::event::ElementState::Released => {
                        self.on_event(Some(event::Event::KeyRelease {
                            key: keycode::KeyCode::convert(
                                input
                                    .virtual_keycode
                                    .unwrap_or(glutin::event::VirtualKeyCode::Space),
                            ),
                        }));
                    }
                },
                glutin::event::WindowEvent::MouseInput { state, button, .. } => match state {
                    glutin::event::ElementState::Pressed => {
                        self.on_event(Some(event::Event::MouseButtonPress {
                            mouse_button: mousecode::MouseCode::convert(button),
                        }));
                    }
                    glutin::event::ElementState::Released => {
                        self.on_event(Some(event::Event::MouseButtonRelease {
                            mouse_button: mousecode::MouseCode::convert(button),
                        }));
                    }
                },
                glutin::event::WindowEvent::MouseWheel { delta, .. } => match delta {
                    glutin::event::MouseScrollDelta::PixelDelta(pos) => {
                        self.on_event(Some(event::Event::MouseScroll { x: pos.x, y: pos.y }));
                    }
                    glutin::event::MouseScrollDelta::LineDelta(x, y) => {
                        self.on_event(Some(event::Event::MouseScroll {
                            x: x as f64,
                            y: y as f64,
                        }));
                    }
                },
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    self.on_event(Some(event::Event::MouseMove {
                        x: position.x,
                        y: position.y,
                    }));
                }
                // glutin::event::WindowEvent::ModifiersChanged(modifiers) => {
                //
                // }
                _ => {}
            },
            _ => (),
        }
    }

    pub fn run(mut self, event_loop: glutin::event_loop::EventLoop<()>) {
        event_loop.run(move |ev, _, control_flow| {
            let mut target = self.display.draw();

            target.clear_color(0.2, 0.2, 0.5, 1.0);

            for layer in self.layer_stack.iter_mut() {
                layer.on_update(&self.input);
            }

            for layer in self.layer_stack.iter_mut() {
                layer.on_egui_render(
                    &self.input,
                    &self.display,
                    &mut target,
                    &ev,
                    control_flow,
                    &mut self.egui_glium,
                );
            }

            match target.finish() {
                Ok(_) => {}
                Err(why) => error!("Drawing error: {}", why),
            }

            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            self.handle_event(ev, control_flow);
        });
    }
}
