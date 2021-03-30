
// based on earlier foundations : TRY 1: Basic Pub/Sub Messaging Kernel Example
// Desmond Germans, Ph.D; www.germansmedia.nl

use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

pub type SID = u64;

#[derive(Clone)]
pub enum Message {
	//Stop,
	//Start,
    Subscribe(SID,String),
    //Unsubscribe(SID,String),
	Event(String,String),
	Add(ServiceBuilder),
}

pub trait Serviceable {
    fn name(&self) -> &str;
	fn stop(&self);
	fn start(&self, sid: SID, send: &Sender<Message>, recv: &Receiver<Message> );
}

pub type ServiceBuilder = fn() -> Box<dyn Serviceable>;

pub struct Kernel {
	pub send: Sender<Message>,
	pub recv: Receiver<Message>,
}

impl Kernel {
	pub fn new(services: &[ServiceBuilder] ) -> Kernel {
		let (send,recv) = unbounded::<Message>();
		let kernel = Kernel { send:send.clone(), recv:recv.clone() };

		// first one MUST be broker; start it now
		let _ = services[0]().start(0,&send.clone(),&recv.clone());

		// add the rest via broker
		for i in 1..services.len() {
			let _ = send.send(Message::Add(services[i]));
		}

		kernel
	}
	//pub fn add(&self,creator: ServiceBuilder) {
	//	let _ = self.send.send(Message::Add(creator));
	//}
}

