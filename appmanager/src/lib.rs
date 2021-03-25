
// TRY 1: Basic Pub/Sub Messaging Kernel Example
// Desmond Germans, Ph.D; www.germansmedia.nl

// This is rougly based on ROS Pub/Sub, after what was explored earlier.

// Notes:
// - Exclusively reactive modules based on message handlers alone might not be the way forward, there is also need for active loops and checks.
// - This implementation currently still misses a timer tick.


use {
    std::{
        thread,
        sync::mpsc::{
            Sender,
            Receiver,
            channel,
        },
        collections::{
            HashSet,
            HashMap,
        },
        cell::RefCell,
    }
};

// Module ID
pub type MID = u64;

// The generic handler type
pub type Handler = fn(MID,Message,&Sender<Message>);

// Application-specific messages - TODO will remove later and make generic
#[derive(Clone)]
pub enum AppMessage {
    VideoFrame,               // hypothetical video frame
    FaceCoords,               // hypothetical face coordinates
}

// Overall messaging structure
#[derive(Clone)]
pub enum Message {
    Startup,                  // initialize the module
    Shutdown,                 // shut down the module
    Tick,                     // timer tick signal (just for this example)
    Subscribe(MID,String),    // "subscribe me to a topic"
    Unsubscribe(MID,String),  // "unsubscribe me from a topic"
    App(String,AppMessage),   // application-specific message to/from a topic
}

// A module
pub struct Module {
    pub name: String,                             // name of the module
    pub mid: MID,                                 // unique module ID
    pub subscriptions: RefCell<HashSet<String>>,  // topics this module receiving from
    pub in_tx: Sender<Message>,                   // broker-side sender to this module
}

// The broker owns the modules, so this API is from the broker's point of view
impl Module {
    pub fn new(name: &str,mid: MID,out_tx: Sender<Message>,handler: Handler) -> Module {

        println!("Creating module '{}'",name);

        // create channel from broker to module
        let (in_tx,in_rx) = channel::<Message>();

        // because of move semantics
        let local_name = name.to_string();

// more like handler(mid,message,&out_tx)
//  and then it spawns any threads and returns

        // start thread, pass incoming messages to the handler
        thread::spawn(move || {

            println!("started thread for module '{}'",local_name);

            // blocking wait for a message
            while let Ok(message) = in_rx.recv() {

                // and execute it
                handler(mid,message,&out_tx);
            }
        });

        // return the running module to the broker
        Module {
            name: name.to_string(),
            mid: mid,
            subscriptions: RefCell::new(HashSet::new()),
            in_tx: in_tx,
        }    
    }
}

///////////////////////////////////////////////////////////////////////////////////
// AppManager
///////////////////////////////////////////////////////////////////////////////////

pub struct AppManager {
	pub modules: HashMap<MID,Module>,
	pub out_tx: Sender<Message>,
	pub out_rx: Receiver<Message>,
}

type FuncType = fn(MID,Message,&Sender<Message>);
//&dyn 

impl AppManager {

	pub fn new() -> AppManager {

	    // create channel from all modules to broker
	    let (out_tx,out_rx) = channel::<Message>();

	    // all currently running modules
	    let mut modules = HashMap::<MID,Module>::new();

		AppManager { modules: HashMap::new(), out_tx:out_tx, out_rx:out_rx }
	}

	pub fn add(&mut self, mid: MID, name: &str, handler: FuncType ) {
	    self.modules.insert(mid,Module::new(name,mid,self.out_tx.clone(),handler));
	}

	pub fn run(&mut self) {

	    // send startup to all modules
	    for module in &self.modules {
	        module.1.in_tx.send(Message::Startup).expect(&format!("broker: failed to send Startup to module {} ('{}')",module.1.mid,module.1.name));
	    }

	    // still missing: timer

	    // flush incoming messages
	    while let Ok(message) = self.out_rx.recv() {

	        match message {

	            // module wants to subscribe to a topic
	            Message::Subscribe(mid,topic) => {
	                println!("broker: subscribing module {} ('{}') to topic '{}'",mid,self.modules[&mid].name,topic);
	                self.modules[&mid].subscriptions.borrow_mut().insert(topic);
	            },

	            // module wants to unsubscribe from a topic
	            Message::Unsubscribe(mid,topic) => {
	                println!("broker: unsubscribing module {} ('{}') from topic '{}'",mid,self.modules[&mid].name,topic);
	                self.modules[&mid].subscriptions.borrow_mut().remove(&topic);
	            },

	            // module sends message to a topic
	            Message::App(topic,app_message) => {
	                for target in &self.modules {
	                    if target.1.subscriptions.borrow_mut().contains(&topic) {
	                        target.1.in_tx.send(
	                            Message::App(topic.clone(),app_message.clone())
	                        ).expect(&format!("broker: failed to send AppMessage to module {} ('{}')",target.1.mid,target.1.name));
	                    }
	                }
	            },

	            // discard anything else
	            _ => { },
	        }
	    }

	}
	pub fn query() {
		println!("queried");
	}
	pub fn remove() {
	}
	pub fn start() {
	}
	pub fn stop() {
	}
	pub fn publish() {
		// publish a message to all listeners with a matching string
	}
	pub fn listen() {
		// listen to any published messages matching a string
	}
	pub fn connect() {
		// connect directly to another app
	}
	pub fn disconnect() {
		// disconnect from an app
	}
}


/*

There are lots of pubsub libraries out there...

CrossBeam is pretty much the same
https://docs.rs/crossbeam-channel/0.4.0/crossbeam_channel/index.html

Futures is more like what I am used to from javascript but it feels a bit newish.
https://docs.rs/futures/0.3.13/futures/
//https://docs.rs/flo_stream/0.6.0/flo_stream/

This again requires a receiver to listen to each channel; I want a single channel pattern.
https://docs.rs/pub-sub/2.0.0/pub_sub/

This is cosmetically a nicer pattern; library is by god knows who
https://github.com/fuchsnj/rust_pubsub

zeromq and nanomsg are kinda heavy for thread based communication

there are idiomatic ways to send data
https://stackoverflow.com/questions/59075477/what-are-idiomatic-ways-to-send-data-between-threads


notes: I could arguably use lazy static to make this resource more widely available

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref APPS: HashMap<String,String> = {
        let mut map = HashMap::new();
        map
    };
}

pub fn find_color_lazy_static(name: &str) -> Option<Striong> {
    APPS.get(name.to_lowercase().as_str()).cloned()
}
*/

