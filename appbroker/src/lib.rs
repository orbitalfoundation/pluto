
// TRY 1: Basic Pub/Sub Messaging Kernel Example
// Desmond Germans, Ph.D; www.germansmedia.nl

//use std::thread;

use std::collections::HashSet;
use std::collections::HashMap;
use std::cell::RefCell;

use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

// Application-specific messages (for now) <- remove

#[derive(Clone)]
pub enum AppMessage {
    VideoFrame,               // hypothetical video frame
    FaceCoords,               // hypothetical face coordinates
}

///
/// App ID <- TODO is there really a point to app ids?
///

pub type AID = u64;

///
/// Messages that apps can send
///

#[derive(Clone)]
pub enum Message {
    Startup,                  // initialize the module
    Shutdown,                 // shut down the module
    Tick,                     // timer tick signal (just for this example)
    Subscribe(AID,String),    // "subscribe me to a topic"
    Unsubscribe(AID,String),  // "unsubscribe me from a topic"
    App(String,AppMessage),   // application-specific message to/from a topic
}

///
/// App Custom Logic
///

pub type AppLogic = fn(AID,&Receiver<Message>,&Sender<Message>);

///
/// A structure to remember properties of each app
/// The broker owns the apps, so this API is from the broker's point of view
///

pub struct AppWrapper {
    pub aid: AID,                                 // unique id of each app
    pub name: String,                             // unique name of each app
    pub asend: Sender<Message>,                   // a channel to send messages to the app
    pub subscriptions: RefCell<HashSet<String>>,  // topics this app is listening to
}

///
/// An App Broker that implements a pubsub mechanic between what we call "apps"
///

pub struct AppBroker {
	pub apps: HashMap<AID,AppWrapper>,
	pub bsend: Sender<Message>,
	pub brecv: Receiver<Message>,
}

impl AppBroker {

	///
	/// build a copy of the broker on demand
	///

	pub fn new() -> AppBroker {

	    // build send and receive channels for traffic to broker
	    let (bsend,brecv) = unbounded::<Message>();

	    // return a fresh broker
		let broker = AppBroker { apps: HashMap::new(), bsend:bsend, brecv:brecv };

		// TODO add a broker event handler which provides access to global services
	    // TODO rather than the broker being quite so special it could be just another app
	    // and we could share it as a default socket or connection that all other apps have
	    // so that they can get access to registry services such as finding each other


		broker
	}

	///
	/// add an app
	///

	pub fn add(&mut self, name: &str, applogic: AppLogic ) {

		// grant each app an ID <- TODO prevent collisions
        let aid: AID = rand::random::<AID>();

        // each app gets comms
	    let (asend,arecv) = unbounded::<Message>();

		// let app fire off threads or whatever it wants
		applogic(aid,&arecv,&self.bsend);

        // remember app

        println!("broker: registered new app name='{}' with id={}",name,aid);

        let name = name.to_string();
        let wrapper = AppWrapper {
            aid: aid,
            name: name,
            asend: asend,
            subscriptions: RefCell::new(HashSet::new()),
        };

		self.apps.insert(aid, wrapper );
	}

	///
	/// run forever
	/// TODO i feel like this whole broker messaging component could ALSO just be another app or module...
	///

	pub fn run(&mut self) {

		// send a startup to all apps... just an idea really... does it make sense?
	    for app in &self.apps {
	        let _res = app.1.asend.send(Message::Startup);
	    }

	    // handle messages forever
	    while let Ok(message) = self.brecv.recv() {

	        match message {

	            // subscribe app to topic
	            Message::Subscribe(aid,topic) => {
	                println!("broker: subscribing app {} ('{}') to topic '{}'",aid,self.apps[&aid].name,topic);
	                self.apps[&aid].subscriptions.borrow_mut().insert(topic);
	            },

	            // unsubscribe app from topic
	            Message::Unsubscribe(aid,topic) => {
	                println!("broker: unsubscribing app {} ('{}') from topic '{}'",aid,self.apps[&aid].name,topic);
	                self.apps[&aid].subscriptions.borrow_mut().remove(&topic);
	            },

	            // route message to apps that are subscribed to a topic
	            // TODO note that the messages are cloned right now... which is expensive....
	            Message::App(topic,app_message) => {
	                for target in &self.apps {
	                    if target.1.subscriptions.borrow_mut().contains(&topic) {
	                        let _res = target.1.asend.send(Message::App(topic.clone(),app_message.clone()));
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

There are lots of messaging libraries out there...

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

