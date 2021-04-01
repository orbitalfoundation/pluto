
use crossbeam::channel::*;
use crate::kernel::*;

#[derive(Clone)]
pub struct Broker {
}
impl Broker {
	pub fn new() -> Box<dyn Serviceable> {
		Box::new(Self{})
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
			let mut registry = std::collections::HashMap::<SID,ServiceWrapper>::new();
		    while let Ok(message) = recv.recv() {
		    	match message {

		            Message::Subscribe(sid,topic) => {
						// TODO if entry doesn't exist yet it should be manufactured... to allow out of order
		                println!("Broker: subscribing app {} ('{}') to topic '{}'",sid,registry[&sid].name,topic);
		                registry[&sid].subscriptions.borrow_mut().insert(topic);
		            },

		            Message::Unsubscribe(sid,topic) => {
		                println!("Broker: unsubscribing app {} ('{}') from topic '{}'",sid,registry[&sid].name,topic);
		                registry[&sid].subscriptions.borrow_mut().remove(&topic);
		            },

		            Message::Event(topic,data) => {
		                for target in &registry {
		                    if target.1.subscriptions.borrow_mut().contains(&topic) {
		                        let _res = target.1.send.send(Message::Event(topic.clone(),data.clone()));
		                    }
		                }
		            },

		    		Message::Add(service) => {

				        let sid: SID = rand::random::<SID>();
						let (localsend,localrecv) = unbounded::<Message>();
		    			let instance = service();
		    			let name = instance.name();

						let wrapper = ServiceWrapper {
							sid: sid,
							name: name.to_string(),
							send: localsend,
							subscriptions: std::cell::RefCell::new(std::collections::HashSet::new()),
						};
						registry.insert(sid,wrapper);

		    			instance.start(sid,&send.clone(),&localrecv);

		    		},

		    		Message::Channel(sid,name,channel) =>{
						let wrapper = ServiceWrapper {
							sid: sid,
							name: name.to_string(),
							send: channel,
							subscriptions: std::cell::RefCell::new(std::collections::HashSet::new()),
						};
						registry.insert(sid,wrapper);
		    		},

		    	}
		    }
		});
	}
}
