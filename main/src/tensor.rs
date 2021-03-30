
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use crate::kernel::*;

pub struct Tensor {}
impl Tensor {
	pub fn new() -> Box<dyn Serviceable> {
		Box::new(Tensor {})
	}
}
impl Serviceable for Tensor {
    fn name(&self) -> &str { "Tensor" }
	fn stop(&self) {}
	fn start(&self, _sid: SID, send: &Sender<Message>, recv: &Receiver<Message> ) {
		let send = send.clone();
		let recv = recv.clone();
		let name = self.name();
		let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {

		    send.send(
		        Message::Subscribe(_sid,"/frames".to_string())
		    ).expect("Tensor: failed to subscribe to topic '/frames'.");

	        while let Ok(message) = recv.recv() {
			    match message {
			    	Message::Event(topic,data) => {
			    		println!("Tensor: got message {} {}",topic, data);
						send.send(
							Message::Event("/faces".to_string(),"hi".to_string())
						).expect("face_detector: failed to send video frame!");
			    	},
			        _ => { },
			    }
	        }
		});
	}
}

