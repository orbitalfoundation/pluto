
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
		wasm::Wasm::new,
		display::Display::new,	// <- note this last service is greedy and captures the main thread, never returning... it's a winit issue and needs more thought
	];

	kernel::Kernel::new( &services );
}
