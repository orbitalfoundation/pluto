
// based on earlier foundations : TRY 1: Basic Pub/Sub Messaging Kernel Example
// Desmond Germans, Ph.D; www.germansmedia.nl

use crossbeam::channel::*;

pub type SID = u64;

#[derive(Clone)]
pub enum Message {

	// register a new channel that can receive traffic
	Channel(SID,String,Sender<Message>),

	// listen to any traffic matching a string
    Subscribe(SID,String),
    Unsubscribe(SID,String),

    // Send an event to any traffic matching a string
	Event(String,String),

	// Dynamically build a service at runtime in the broker (not used right now)
	Add(ServiceBuilder),

	// TODO examine - what i really want to do is send an actual trait instance...
	//Add2(&Serviceable),
	//Add2(Box<&Serviceable>),
	//AddInstance(Box<dyn Serviceable>),
	// https://www.reddit.com/r/rust/comments/7q3bz8/trait_object_with_clone/
	// https://www.reddit.com/r/rust/comments/8q0602/a_generic_trait_for_cloning_boxed_trait_objects/
}

pub trait Serviceable: ServiceableClone {
    fn name(&self) -> &str;
	fn stop(&self);
	fn start(&self, sid: SID, send: &Sender<Message>, recv: &Receiver<Message> );
}

pub trait ServiceableClone {
	fn clone_box(&self) -> Box<dyn Serviceable>;
}

impl<T> ServiceableClone for T
	where T: 'static + Serviceable + Clone
{
	fn clone_box(&self) -> Box<dyn Serviceable> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn Serviceable> {
	fn clone(&self) -> Box<dyn Serviceable> {
		self.clone_box()
	}
}

pub struct ServiceWrapper {
	pub sid: SID,
    pub name: String,
    pub send: Sender<Message>,
    pub subscriptions: std::cell::RefCell<std::collections::HashSet<String>>,
}

pub type ServiceBuilder = fn() -> Box<dyn Serviceable>;

pub struct Kernel {}

impl Kernel {

	pub fn new(services: &[ServiceBuilder] ) -> Kernel {
		let (send,recv) = unbounded::<Message>();

		// broker is special; pass it global send/recv channels
		let _ = services[0]().start(0,&send.clone(),&recv.clone());

		// add the rest
		for i in 1..services.len() {
	        let sid: SID = rand::random::<SID>();
			let (localsend,localrecv) = unbounded::<Message>();
			let instance = services[i]();
			let name = instance.name();
			let _ = send.send(Message::Channel(sid,name.to_string(),localsend));
			instance.start(sid,&send.clone(),&localrecv);
		}

		Kernel {}
	}
}

