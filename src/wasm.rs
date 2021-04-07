
use crossbeam::channel::*;
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

#[derive(Clone)]
pub struct Wasm {}
impl Wasm {
    pub fn new() -> Box<dyn Serviceable> {
        Box::new(Self{})
    }
}
impl Serviceable for Wasm {
    fn name(&self) -> &str { "Wasm" }
	fn stop(&self) {}
	fn start(&self, name: String, _sid: SID, send: Sender<Message>, recv: Receiver<Message> ) {
        let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {
	   	   let _ = wasm2(name,send,recv);
        });
	}
}

fn wasm2(name: String, send: Sender<Message>,_recv:Receiver<Message>) -> Result<(), Box<dyn Error>> {

    let store = Store::default();

    println!("Wasm: fetching url {}",name);
    let module = Module::from_file(store.engine(), name)?;

    // TODO -> inbound traffic - write support for
    // what is a pattern for sending messages *TO* the wasm blob?
    // if it is a thread it may never return... it could poll a message queue?
    // while recv.try_recv -> add to queue that wasm blob can look at?

    // TODO -> outbound traffic improve?
    // in this test/approach there are functions that the wasm blob can call to talk to system capabilities
    // there are no closures here
    let callback_func = Func::wrap(&store,move || {
        println!("wasm: got a call from wasm blob");
        let _ = send.send(Message::Event("/camera".to_string(),"WASM->Camera: Give me a Frame".to_string()));
        let _ = send.send(Message::Event("/display".to_string(),"WASM->Display: Show Frame".to_string()));
    });
    let imports = [callback_func.into()];

    // fire off entry point - since this has no guarantee of returning it should be a thread
    let instance = Instance::new(&store, &module, &imports)?;
    let startup = instance.get_typed_func::<(),()>("startup")?;
    startup.call(())?;

    println!("wasm: done test");
    // TODO think about not ending

    Ok(())
}

