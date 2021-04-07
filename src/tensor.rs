
use crossbeam::channel::*;
use crate::kernel::*;

#[derive(Clone)]
pub struct Tensor {}
impl Tensor {
	pub fn new() -> Box<dyn Serviceable> {
		Box::new(Self{})
	}
}
impl Serviceable for Tensor {
    fn name(&self) -> &str { "Tensor" }
	fn stop(&self) {}
	fn start(&self, _name: String, _sid: SID, send: Sender<Message>, recv: Receiver<Message> ) {
		let send = send.clone();
		let recv = recv.clone();
		let name = self.name();
		let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {

			// in this sketch the pretend tensor module listens to ALL camera frames and looks for faces as a built in capability (like recognizing qr codes)
			// TODO arguably like the camera service this should only work on a given frame and only pipe back to the a specified caller
			let message = Message::Subscribe(_sid,"/frames".to_string());
		    send.send(message).expect("error");

	        while let Ok(message) = recv.recv() {
			    match message {
			    	Message::Event(topic,data) => {
			    		println!("Face: Received: {} {}",topic, data);
			    		let message = Message::Event("/display".to_string(),"[Face->Display: here is a face]".to_string());
						send.send(message).expect("error");
			    	},
			        _ => { },
			    }
	        }
		});
	}
}

