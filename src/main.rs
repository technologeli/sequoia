use glium::glutin::{self, event_loop::ControlFlow};

mod sequoia;

// macro_rules! debug {
//     ($expression:expr) => {
//         if cfg!(debug_assertions) {
//             $expression
//         }
//     };
// }

struct ExampleLayer {
    tree: Tree<String>,
}
impl sequoia::layer::Layer for ExampleLayer {
    fn on_egui_render(
        &mut self,
        input: &sequoia::input::Input,
        display: &glium::Display,
        target: &mut glium::Frame,
        event: &glium::glutin::event::Event<()>,
        control_flow: &mut glium::glutin::event_loop::ControlFlow,
        egui_glium: &mut egui_glium::EguiGlium,
    ) {
        let repaint_after = egui_glium.run(display, |egui_ctx| {
            DockArea::new(&mut self.tree)
                .style(Style::from_egui(egui_ctx.style().as_ref()))
                .show(egui_ctx, &mut TabViewer {});
            // egui::SidePanel::left("side_panel").show(egui_ctx, |ui| {
            //
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

    let mut app = Box::new(sequoia::application::Application::new(display, &event_loop));
    app.push_layer(Box::new(ExampleLayer::default()));
    app.run(event_loop);
}

use egui_dock::{DockArea, NodeIndex, Style, Tree};

struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}

impl Default for ExampleLayer {
    fn default() -> Self {
        let mut tree = Tree::new(vec!["tab1".to_owned(), "tab2".to_owned()]);

        // You can modify the tree before constructing the dock
        let [a, b] = tree.split_right(NodeIndex::root(), 0.3, vec!["tab3".to_owned()]);
        let [_, _] = tree.split_below(a, 0.7, vec!["tab4".to_owned()]);
        let [_, _] = tree.split_below(b, 0.5, vec!["tab5".to_owned()]);

        Self { tree }
    }
}

// In egui_glium.run, create a dock using the tree
// The tree is a series of nodes that are split denoted by T, typically String
// TabViewer is a trait that requires title and ui functions
// You implement different tab viewers for different tabs, which are user-defined structs
