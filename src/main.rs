

// seems like a language defect if these macros cannot be down at appropriate scope - pollutes stuff that just does not need to know
#[macro_use] extern crate objc;

// seems like a language defect if these macros cannot be down at appropriate scope - pollutes stuff that just does not need to know
//#[macro_use]
//extern crate simple_counter;

// this is too monsterously huge to deal with - was testing it up here due to embedded macros - but it has too many pedantic issues and errors
//include!("../avtest/avtestbind.in");

// all built in pieces

mod kernel;
mod broker;

mod camera;
mod tensor;
mod display;
mod wasm;

///
/// orbital entry point
///

pub fn main() {

	let services = [
		broker::Broker::new,
		camera::Camera::new,
		tensor::Tensor::new,
		display::Display::new, // <- note for now this service never returns
	];

	kernel::Kernel::new( &services );
}

