
// based on earlier foundations : TRY 1: Basic Pub/Sub Messaging Kernel Example
// Desmond Germans, Ph.D; www.germansmedia.nl

use crossbeam::channel::*;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

///
/// SID
/// Service ID
/// Each service gets a UUID for now for hashed lookup
/// TODO later may just use name? May have to grant names or somehow prevent collisions
///

pub type SID = u64;

///
/// Message
/// Broker gets all messages for now.
/// Some messages are routed to other services by broker.
/// TODO Later services can build direct relationships.
///

#[derive(Clone)]
pub enum Message {
	Share(Arc<Mutex<Box<[u32;262144]>>>),

	// register a new channel that can receive traffic
	Channel(SID,String,Sender<Message>),

	// listen to any traffic matching a string
    Subscribe(SID,String),
    Unsubscribe(SID,String),

    // Broker Goto - TODO for now special traffic directed at broker is special later perhaps just use ordinary events?
    BrokerGoto(String),

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
	fn start(&self, name: String, sid: SID, send: Sender<Message>, recv: Receiver<Message> );
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

		// broker is expected to be first; start it with vanilla channel endpoints
		let _ = services[0]().start("broker".to_string(),0,send.clone(),recv.clone());

		// start the rest; pass inbound channel and outbound broker channel, also register a channel endpoint with the broker
		for i in 1..services.len() {
	        let sid: SID = rand::random::<SID>();
			let (localsend,localrecv) = unbounded::<Message>();
			let instance = services[i]();
			let name = instance.name().to_string();
			let _ = send.send(Message::Channel(sid,name.clone(),localsend));
			instance.start(name,sid,send.clone(),localrecv);
		}

		Kernel {}
	}
}

