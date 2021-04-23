
use crossbeam::channel::*;
use crate::kernel::*;
use crate::wasm::*;

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;


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
	fn start(&self, _name: String, _sid: SID, send: Sender<Message>, recv: Receiver<Message> ) {		
		let name = self.name();
		let send = send.clone();
		let recv = recv.clone();
		println!("kernel broker starting {}",_sid);

		let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {
			let mut registry = std::collections::HashMap::<SID,ServiceWrapper>::new();
		    while let Ok(message) = recv.recv() {
		    	match message {

		            Message::Subscribe(sid,topic) => {
		            	if !registry.contains_key(&sid) {
			                println!("Broker: forcing entry for non-existent app {} ('{}') to topic '{}'",sid,registry[&sid].name,topic);
							let (_trashsend,_trashreceive) = unbounded::<Message>();
							let wrapper = ServiceWrapper {
								sid: sid,
								name: "no name yet".to_string(),
								send: _trashsend,
								subscriptions: std::cell::RefCell::new(std::collections::HashSet::new()),
							};
							registry.insert(sid,wrapper);
		            	}
		                println!("Broker: subscribing app {} ('{}') to topic '{}'",sid,registry[&sid].name,topic);
		                registry[&sid].subscriptions.borrow_mut().insert(topic);
		            },

		            Message::Unsubscribe(sid,topic) => {
		                println!("Broker: unsubscribing app {} ('{}') from topic '{}'",sid,registry[&sid].name,topic);
		                registry[&sid].subscriptions.borrow_mut().remove(&topic);
		            },

		            // hack, forward share objects...
		            Message::Share(sharedmemory) => {
		            	// repost event objects 
		                for target in &registry {
		                    if target.1.subscriptions.borrow_mut().contains(&"/display".to_string()) {
			                    //let mut ptr = sharedmemory.lock().unwrap();
				                //let mut sharedmemory = Arc::new(Mutex::new(Box::new(ptr)));
		                        let _res = target.1.send.send(Message::Share(sharedmemory));
		                        break;
		                    }
		                }
		            },

		            Message::Event(topic,data) => {
		            	// repost event objects 
		                for target in &registry {
		                    if target.1.subscriptions.borrow_mut().contains(&topic) {
		                        let _res = target.1.send.send(Message::Event(topic.clone(),data.clone()));
		                    }
		                }
		            },

		    		Message::Add(service) => {

		    			// we used to let the broker add the whole service - but there's a main thread issue with display so right now this approach is not used - see channel below
				        let sid: SID = rand::random::<SID>();
						let (localsend,localrecv) = unbounded::<Message>();
		    			let instance = service();
		    			let name = instance.name().to_string();

						let wrapper = ServiceWrapper {
							sid: sid,
							name: name.clone(),
							send: localsend,
							subscriptions: std::cell::RefCell::new(std::collections::HashSet::new()),
						};
						registry.insert(sid,wrapper);

		    			instance.start(name,sid,send.clone(),localrecv);

		    		},

		    		Message::Channel(sid,name,channel) =>{
		            	if !registry.contains_key(&sid) {
							println!("Broker: added channel for {} {}",sid,name);
							let wrapper = ServiceWrapper {
								sid: sid,
								name: name,
								send: channel,
								subscriptions: std::cell::RefCell::new(std::collections::HashSet::new()),
							};
							registry.insert(sid,wrapper);
						} else {
							println!("Broker: revising existing channel for {} {}",sid,name);
							registry.remove(&sid);
							let subscriptions = registry[&sid].subscriptions.clone();
							let wrapper = ServiceWrapper {
								sid: sid,
								name: name.clone(),
								send: channel,
								subscriptions: subscriptions,
							};
							registry.insert(sid,wrapper);
						}
		    		},

		            Message::BrokerGoto(url) => {

		            	if url.len() < 1 {
		            		return
		            	}

					    let mut found = false;

					    for (_key, value) in &registry {
					        if value.name.eq(&url) {
					        	println!("found {} {}", value.name, url );
					        	found = true
					        }
					    }

					    if found == true {
					    	return
					    }

				        let sid: SID = rand::random::<SID>();
						let (localsend,localrecv) = unbounded::<Message>();
		    			let instance = Wasm::new();
		    			let name = url;

						println!("Broker: added channel for {} {}",sid,name);

						let wrapper = ServiceWrapper {
							sid: sid,
							name: name.clone(),
							send: localsend,
							subscriptions: std::cell::RefCell::new(std::collections::HashSet::new()),
						};
						registry.insert(sid,wrapper);
		    			instance.start(name,sid,send.clone(),localrecv);

		            },

		            _ => {

		            }
		    	}
		    }
		});
	}
}
