
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
		wasm::Wasm::new,
		tensor::Tensor::new,
		display::Display::new,
	];

	kernel::Kernel::new( &services );


    std::thread::sleep(std::time::Duration::from_millis(10000));

}


// - camera: can the camera data be fetched better?
// - camera: don't publish frames, rather have a direct pipe of some kind to caller or wait for commands


// - wasm: try move burden of work there