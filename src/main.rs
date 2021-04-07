
mod kernel;
mod broker;

mod camera;
mod tensor;
mod display;
mod wasm;

pub fn main() {

	let services = [
		broker::Broker::new,
		camera::Camera::new,
		tensor::Tensor::new,
		display::Display::new, // <- note for now this service never returns
	];

	kernel::Kernel::new( &services );
}

