use crate::core::engine::objects::graphics::GraphicsObjects;

#[derive(Debug, Default)]
pub struct TestComponent {
	gob: GraphicsObjects,
	texture: GraphicsObjects,
}

const VERTICES: [f32; 12] = [
    // Positions   
    -0.5, -0.5, 0.0,  // Bottom-left
     0.5, -0.5, 0.0,  // Bottom-right
    -0.5,  0.5, 0.0,  // Top-left
     0.5,  0.5, 0.0,  // Top-right
];

const TEX_COORDS: [f32; 8] = [
	0.0, 0.0,
	1.0, 0.0,
	0.0, 1.0,
	1.0, 1.0,
];

const INDICES: [u32; 6] = [
    0, 1, 2, // First triangle
    1, 3, 2, // Second triangle
];

impl TestComponent {
	pub fn init(&mut self) {
		self.gob.generate_vao();
		self.gob.generate_vbo();
		self.gob.generate_ebo();

		self.texture.generate_vbo();

		self.gob.bind_vao();
		self.gob.bind_vbo();

		unsafe {
			// Bind and fill VBO
			gl::BindBuffer(gl::ARRAY_BUFFER, self.gob.vbo);
			gl::BufferData(gl::ARRAY_BUFFER, (VERTICES.len() * std::mem::size_of::<f32>()) as isize, VERTICES.as_ptr() as *const _, gl::STATIC_DRAW);	

			// Define vertex attribute pointers (assuming positions only, 3 floats per vertex)
			gl::VertexAttribPointer(
				0, // attribute index
				3, // number of components (x, y, z)
				gl::FLOAT, // type of components
				gl::FALSE, // normalized
				0, // stride
				std::ptr::null(), // pointer to the start
			);
			gl::EnableVertexAttribArray(0);

			// Bind and fill EBO
			self.gob.bind_ebo();
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gob.ebo);
			gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (INDICES.len() * std::mem::size_of::<u32>()) as isize, INDICES.as_ptr() as *const _, gl::STATIC_DRAW);
			
			self.texture.bind_vbo();
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(TEX_COORDS.len() * std::mem::size_of::<[f32; 2]>()) as isize,
				TEX_COORDS.as_ptr() as *const std::ffi::c_void,
				gl::STATIC_DRAW,
			);
			gl::VertexAttribPointer(
				1,
				2,
				gl::FLOAT,
				gl::FALSE,
				0,
				std::ptr::null(),
			);
			gl::EnableVertexAttribArray(1);
		}

		self.gob.unbind_vbo();
		self.gob.unbind_vao();
	}

	pub fn draw(&mut self) {
		// Draw the square
		unsafe {
            self.gob.bind_vao();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
	}
}