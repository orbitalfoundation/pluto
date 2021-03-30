
use std::collections::HashSet;
use std::collections::HashMap;
use std::cell::RefCell;
use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use crate::kernel::*;

struct ServiceWrapper {
	pub sid: SID,
    pub name: String,
    pub send: Sender<Message>,
    pub subscriptions: RefCell<HashSet<String>>,
}

pub struct Broker {
}
impl Broker {
	pub fn new() -> Box<dyn Serviceable> {
		Box::new(Broker {})
	}
}
impl Serviceable for Broker {
    fn name(&self) -> &str { "broker" }
	fn stop(&self) {}
	fn start(&self, _sid: SID, send: &Sender<Message>, recv: &Receiver<Message> ) {		
		let name = self.name();
		let send = send.clone();
		let recv = recv.clone();
		println!("kernel broker starting {}",_sid);

		let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {
			let mut registry = HashMap::<SID,ServiceWrapper>::new();
		    while let Ok(message) = recv.recv() {
		    	match message {
		    		//Message::Stop => {
		    		//},
		    		//Message::Start => {
		    		//},
		            Message::Subscribe(sid,topic) => {
		                println!("broker: subscribing app {} ('{}') to topic '{}'",sid,registry[&sid].name,topic);
		                registry[&sid].subscriptions.borrow_mut().insert(topic);
		            },
		            //Message::Unsubscribe(sid,topic) => {
		            //    println!("broker: unsubscribing app {} ('{}') from topic '{}'",sid,registry[&sid].name,topic);
		            //    registry[&sid].subscriptions.borrow_mut().remove(&topic);
		            //},
		            Message::Event(topic,data) => {
		            	println!("broker: forwarding msg");
		                for target in &registry {
		                    if target.1.subscriptions.borrow_mut().contains(&topic) {
		                        let _res = target.1.send.send(Message::Event(topic.clone(),data.clone()));
		                    }
		                }
		            },
		    		Message::Add(creator) => {

				        let sid: SID = rand::random::<SID>();
						let (localsend,localrecv) = unbounded::<Message>();
		    			let thing = creator();
		    			let name = thing.name();
		    			thing.start(sid,&send.clone(),&localrecv);

		    			println!("Started handler {}", name );

						let wrapper = ServiceWrapper {
							sid: sid,
							name: name.to_string(),
							send: localsend,
							subscriptions: RefCell::new(HashSet::new()),
						};
						registry.insert(sid,wrapper);
						//registry.insert(name,Box::new(thing));

		    		},
		    		//_ => {},
		    	}
		    }
		});
	}
}
