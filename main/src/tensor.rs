
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
	fn start(&self, _sid: SID, send: &Sender<Message>, recv: &Receiver<Message> ) {
		let send = send.clone();
		let recv = recv.clone();
		let name = self.name();
		let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {

			// there is a race - this below code should not run before this whole fn is in registry
		    std::thread::sleep(std::time::Duration::from_millis(1000));

		    send.send(
		        Message::Subscribe(_sid,"/frames".to_string())
		    ).expect("Tensor: failed to subscribe");

	        while let Ok(message) = recv.recv() {
			    match message {
			    	Message::Event(topic,data) => {
			    		println!("Tensor: changing message {} {}",topic, data);
						send.send(
							Message::Event("/display".to_string(),"hi".to_string())
						).expect("Tensor: failed to send video frame!");
			    	},
			        _ => { },
			    }
	        }
		});
	}
}

