use glium::Surface;


#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn from(position: [f32; 3]) -> Self {
        Self { position }
    }
}

glium::implement_vertex!(Vertex, position);

pub struct VertexArray {
    pub vertex_buffers: Vec<glium::VertexBuffer<Vertex>>,
    pub index_buffer: glium::IndexBuffer<u32>,
}

pub struct Renderer;

impl Renderer {
    pub fn set_clear_color(target: &mut glium::Frame, color: glam::Vec4) {
        target.clear_color(color.x, color.y, color.z, color.w)
    }
    pub fn draw_indexed(
        target: &mut glium::Frame,
        program: &glium::Program,
        vertex_array: &VertexArray,
        view_projection_matrix: glam::Mat4
    ) {
        for vertex_buffer in vertex_array.vertex_buffers.iter() {
            let uniforms = glium::uniform! { u_ViewProjection: view_projection_matrix.to_cols_array_2d() };
            target
                .draw(
                    vertex_buffer,
                    &vertex_array.index_buffer,
                    program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
        }
    }
}

pub struct OrthographicCamera {
    projection_matrix: glam::Mat4,
    view_matrix: glam::Mat4,
    view_projection_matrix: glam::Mat4,
    position: glam::Vec3,
    rotation: f32,
}

impl OrthographicCamera {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, position: glam::Vec3, rotation: f32) -> Self {
        let projection_matrix = glam::Mat4::orthographic_lh(left, right, bottom, top, -1.0, 1.0);

        let transform = glam::Mat4::from_translation(position)
            * glam::Mat4::from_rotation_z(rotation);
        let view_matrix = transform.inverse();
        let view_projection_matrix = projection_matrix * view_matrix;
        Self {
            projection_matrix,
            view_matrix,
            view_projection_matrix,
            position,
            rotation,
        }
    }

    pub fn set_position(&mut self, position: glam::Vec3) {
        self.position = position;
        self.recalculate_view_matrix();
    }
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.recalculate_view_matrix();
    }


    fn recalculate_view_matrix(&mut self) {
        let transform = glam::Mat4::from_translation(self.position)
            * glam::Mat4::from_rotation_z(self.rotation.to_radians());
        self.view_matrix = transform.inverse();
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }

    pub fn view_projection_matrix(&self) -> glam::Mat4 {
        self.view_projection_matrix
    }

    pub fn view_matrix(&self) -> glam::Mat4 {
        self.view_matrix
    }

    pub fn projection_matrix(&self) -> glam::Mat4 {
        self.projection_matrix
    }
}
