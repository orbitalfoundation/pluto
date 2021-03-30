

//use std::time::Duration;
//use std::thread;
//use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use wasmtime::*;
use std::error::Error;
use std::fmt;

use crate::kernel::*;

//////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

//////////////////////////////////////////////////////////////////////////

pub struct Wasm {}
impl Wasm {
	pub fn new() -> Box<dyn Serviceable> {
		Box::new(Wasm {})
	}
}
impl Serviceable for Wasm {
    fn name(&self) -> &str { "Wasm" }
	fn stop(&self) {}
	fn start(&self, _sid: SID, _send: &Sender<Message>, _recv: &Receiver<Message> ) {
		//let send = send.clone();
		//let recv = recv.clone();
		//let name = self.name();
		//let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {
		//});

		let _ = wasm2();
		//	if let Err(e) = wasm2() {
		//		println!("error occured {}",e);
		//	}

	}
}

fn wasm2() -> Result<(), Box<dyn Error>> {

    let store = Store::default();

    // TODO -> another way would be to expose functions to the wasm blob similar to wasi
    // and those function in turn could throw state back up to here

    // define a callback which wasm blob can call to do some work up here
    let callback_func = Func::wrap(&store, || {
        println!("wasm: got a call from wasm blob");
    });

    // add it to a list of imports to ship to the wasm blob
    let imports = [callback_func.into()];

    // load and instance the blob
    let module = Module::from_file(store.engine(), "mywasm.wat")?;
    let instance = Instance::new(&store, &module, &imports)?;

    // get exports
    //let startup = instance.get_func("startup").expect("`startup` was not an exported function");

    let startup = instance.get_typed_func::<(),()>("startup")?;

    // TODO - this blob could chose to simply never return and it can keep calling the callback to do work

    startup.call(())?;

    println!("wasm: done test");

    Ok(())
}

