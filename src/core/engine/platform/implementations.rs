use egui_glfw::glfw::{self, GlfwReceiver, WindowEvent};

#[derive(Clone)]
pub struct Size {
	pub width: u32,
	pub height: u32,
}

impl Size {
	pub fn from(width: u32, height: u32) -> Self {
		Size { width, height }
	}
}

#[derive(Clone, Copy)]
pub struct DisplayOptions {
	pub fit_screen: bool,
	pub fullscreen: bool,
}

#[derive(Clone)]
pub struct WindowOptions {
	pub title: String,
	pub size: Size,
}

impl Default for WindowOptions {
	fn default() -> Self {
		WindowOptions {
			title: String::new(),
			size: Size::from(800, 400)
		}
	}
}

pub struct WindowProperties {
	pub window_options: WindowOptions,
	pub display_options: Option<DisplayOptions>
}

pub struct Window {
	pub glfw: glfw::Glfw,
	pub window: glfw::PWindow,
	pub events: GlfwReceiver<(f64, WindowEvent)>
}