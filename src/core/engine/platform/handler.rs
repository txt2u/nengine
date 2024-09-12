use egui_glfw::glfw;
use super::implementations::{Window, WindowProperties};

impl Window {
	pub fn new(properties: WindowProperties) -> Self {
		let mut glfw = glfw::init(Self::error_callback)
			.expect("Failed to initialize glfw, See platform/window.rs");

		glfw.window_hint(glfw::WindowHint::ContextVersion(4, 4));
		glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
		glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));

		
		let (mut window, events) = glfw.clone()
		.with_primary_monitor(|_, mut m| {
			glfw.create_window(if let Some(screen) = properties.display_options { 
				if screen.fit_screen {
					m.as_mut().expect("Failed to get monitor").get_workarea().2 as u32
				} else {
					properties.window_options.size.width
				}
			} else {
				properties.window_options.size.width
			}, if let Some(screen) = properties.display_options { 
				if screen.fit_screen {
					m.as_mut().expect("Failed to get monitor").get_workarea().3 as u32
				} else {
					properties.window_options.size.height
				}
			} else {
				properties.window_options.size.height
			}, properties.window_options.title.as_str(),
			m.as_mut().map_or(glfw::WindowMode::Windowed, |m| 
				if let Some(screen) = properties.display_options { 
					if screen.fullscreen { 
						glfw::WindowMode::FullScreen(m) 
					} else { 
							glfw::WindowMode::Windowed 
						} 
					} else { 
						glfw::WindowMode::Windowed 
					}
				))
		}).expect("Failed to create GLFW window.");
			
		window.set_key_polling(true);
		window.set_mouse_button_polling(true);
		window.set_cursor_pos_polling(true);

		Window {
			glfw,
			window,
			events
		}
	}

	pub fn should_close(&mut self) -> bool {
		self.window.should_close()
	}

	pub fn initialize_opengl(&mut self) {
		self.glfw.make_context_current(Some(&self.window));
		gl::load_with(|s| self.glfw.get_proc_address_raw(s));

		unsafe {
			self.game_loop();
		}
	}

	fn error_callback(err: glfw::Error, description: String) {
		panic!("GLFW error {:?}: {:?}", err, description);
	}
}