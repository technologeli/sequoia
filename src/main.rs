use glium::glutin::{self, event_loop::ControlFlow};

mod sequoia;

// macro_rules! debug {
//     ($expression:expr) => {
//         if cfg!(debug_assertions) {
//             $expression
//         }
//     };
// }
use egui_dock::{DockArea, NodeIndex, Style, Tree};

struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if tab == "Viewport" {
            // how can i send this to the renderer?
            // let rect = ui.min_rect();
            // info!("{:?}", rect);
        } else {
            ui.label(format!("Content of {tab}"));
            let rect = ui.min_rect();
            let g = 40;
            ui.painter()
                .rect_filled(rect, 0.0, egui::Color32::from_rgb(g, g, g));
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}

struct ExampleLayer {
    tree: Tree<String>,
    vertex_array: sequoia::renderer::VertexArray,
    program: glium::program::Program,
    camera: sequoia::renderer::OrthographicCamera,
    camera_position: glam::Vec3,
    camera_speed: f32,
}

impl ExampleLayer {
    pub fn new(display: &glium::Display) -> Self {
        let mut tree = Tree::new(vec!["Viewport".to_owned()]);

        // You can modify the tree before constructing the dock
        let [a, _] = tree.split_right(NodeIndex::root(), 0.75, vec!["Settings".to_owned()]);
        let [_, _] = tree.split_left(a, 0.25, vec!["Console".to_owned()]);

        let shape = vec![
            sequoia::renderer::Vertex::from([-0.5, -0.5, 0.0]),
            sequoia::renderer::Vertex::from([0.5, -0.5, 0.0]),
            sequoia::renderer::Vertex::from([0.5, 0.5, 0.0]),
            sequoia::renderer::Vertex::from([-0.5, 0.5, 0.0]),
        ];

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &[0, 1, 2, 0, 2, 3],
        )
        .unwrap();

        let vertex_array = sequoia::renderer::VertexArray {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
        };

        let vertex_shader_src = r#"
            #version 330 core
            in vec3 position;
            out vec3 v_Position;
            uniform mat4 u_ViewProjection;
            void main() {
                v_Position = position;
                gl_Position = u_ViewProjection * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330 core
            out vec4 color;
            in vec3 v_Position;
            void main() {
                color = vec4(v_Position * 0.5 + 0.5, 1.0);
            }
        "#;
        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();
        let camera_position = glam::vec3(0.0, 0.0, 0.0);
        let camera =
            sequoia::renderer::OrthographicCamera::new(-1.6, 1.6, -0.9, 0.9, camera_position, 0.0);

        Self {
            tree,
            vertex_array,
            program,
            camera,
            camera_position,
            camera_speed: 0.01,
        }
    }
}

impl sequoia::layer::Layer for ExampleLayer {
    fn on_event(&mut self, event: &mut Option<sequoia::event::Event>) {
        if let Some(e) = event {
            match e {
                sequoia::event::Event::KeyPress { key } => match key {
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn on_update(&mut self, target: &mut glium::Frame, input: &sequoia::input::Input) {
        {
            use sequoia::keycode::KeyCode;
            if input.keys_pressed.contains(&KeyCode::A) {
                self.camera_position.x += self.camera_speed;
            }
            if input.keys_pressed.contains(&KeyCode::D) {
                self.camera_position.x -= self.camera_speed;
            }
            if input.keys_pressed.contains(&KeyCode::W) {
                self.camera_position.y -= self.camera_speed;
            }
            if input.keys_pressed.contains(&KeyCode::S) {
                self.camera_position.y += self.camera_speed;
            }
        }

        sequoia::renderer::Renderer::set_clear_color(target, glam::vec4(0.1, 0.1, 0.1, 1.0));
        self.camera.set_position(self.camera_position);
        // self.camera.set_rotation(45.0);

        // draw the square
        // in the future, program and vertex array are different per shape
        sequoia::renderer::Renderer::draw_indexed(
            target,
            &self.program,
            &self.vertex_array,
            self.camera.view_projection_matrix(),
        );
    }

    fn on_egui_render(
        &mut self,
        _input: &sequoia::input::Input,
        display: &glium::Display,
        target: &mut glium::Frame,
        event: &glium::glutin::event::Event<()>,
        control_flow: &mut glium::glutin::event_loop::ControlFlow,
        egui_glium: &mut egui_glium::EguiGlium,
    ) {
        let repaint_after = egui_glium.run(display, |egui_ctx| {
            let mut s = Style::from_egui(egui_ctx.style().as_ref());
            s.default_inner_margin = egui::style::Margin::default();
            s.tab_background_color = egui::Color32::TRANSPARENT;
            DockArea::new(&mut self.tree)
                .style(s)
                .show(egui_ctx, &mut TabViewer);
            // egui::SidePanel::left("side_panel").show(egui_ctx, |ui| {
            //     ui.visuals_mut().window_fill = egui::Color32::RED;
            //     ui.visuals_mut().panel_fill = egui::Color32::DARK_RED;
            //     ui.visuals_mut().code_bg_color = egui::Color32::DARK_GREEN;
            //     ui.visuals_mut().faint_bg_color = egui::Color32::GREEN;
            //     ui.visuals_mut().extreme_bg_color = egui::Color32::LIGHT_GREEN;
            //     ui.heading("Sequoia");
            //     if ui.button("Click me").clicked() {
            //         if input
            //             .keys_pressed
            //             .contains(&sequoia::keycode::KeyCode::LeftShift)
            //         {
            //             info!("You are holding left shift");
            //         } else {
            //             info!("You are not holding left shift");
            //         }
            //     }
            // });
        });

        *control_flow = if repaint_after.is_zero() {
            display.gl_window().window().request_redraw();
            ControlFlow::Poll
        } else if let Some(repaint_after_instant) =
            std::time::Instant::now().checked_add(repaint_after)
        {
            ControlFlow::WaitUntil(repaint_after_instant)
        } else {
            ControlFlow::Wait
        };

        egui_glium.paint(display, target);

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            // glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            // glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),
            glutin::event::Event::WindowEvent { event, .. } => {
                let event_response = egui_glium.on_event(&event);

                if event_response.repaint {
                    display.gl_window().window().request_redraw();
                }
            }
            glutin::event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached {
                ..
            }) => display.gl_window().window().request_redraw(),
            _ => (),
        }
    }
}

fn main() {
    sequoia::log::init();

    let event_loop = glium::glutin::event_loop::EventLoop::new();

    let size = glutin::dpi::LogicalSize::new(50.0 * 16.0, 50.0 * 9.0);
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Sequoia")
        .with_inner_size(size);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let example_layer = Box::new(ExampleLayer::new(&display));
    let mut app = Box::new(sequoia::application::Application::new(display, &event_loop));
    app.push_layer(example_layer);
    app.run(event_loop);
}

// In egui_glium.run, create a dock using the tree
// The tree is a series of nodes that are split denoted by T, typically String
// TabViewer is a trait that requires title and ui functions
// You implement different tab viewers for different tabs, which are user-defined structs
