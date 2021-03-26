/*

display hardware emulation

Q: What is a good way to represent display output?

In most hardware there are going to be limited or scarce resources such as displays
Typically a display has some kind of outer scope manager doling out privileges

	+ create_a_window() -> returning a handle
	+ draw_into_a_window() -> given a handle and so on

We can probably do the same (although anything we do will be replaced by third parties).

I think we can juse send messages to the display support - basically a DSL.
Note that I *don't want* a fancy DSL in general but I just want something to exercise displays with.

Q: What are good windowing tools for Rust that can be a temporary stand in so we can "show work"?

Kiss3d -> too specialized
Winit -> not powerful enough
Piston -> good enough for now?

Note: There is some kind of event loop constraint on windowing where it has to be "on the main thread":

https://docs.rs/winit/0.20.0-alpha4/winit/event_loop/struct.EventLoop.html

*/

use std::time::Duration;
use std::thread;

use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use appbroker::*;

pub enum Drawable {
	Anything,
	Circle(u32),
	Box(u32,u32,u32,u32),
}
/////////////////////////////////////////////////////////

extern crate piston_window;
use piston_window::*;

///
/// Startup display services
///

pub fn app(_aid: AID, arecv: &Receiver<Message>, bsend: &Sender<Message>)  {

	// a way to talk to windowing internal to this function
	// windowing has an issue where it cannot run in a subthread due to events
	let (wsend, wrecv) = unbounded::<Drawable>();
	wsend.send(Drawable::Anything).unwrap();


	// TODO how bad is this to do?
	let arecv = arecv.clone();
	let bsend = bsend.clone();

    // start thread and look for messages
    thread::spawn(move || {

        // blocking wait for a message
        while let Ok(message) = arecv.recv() {

		    match message {

		        // display does startup, subscribes to face coordinate topic
		        Message::Startup => {
		            println!("display: Startup");
		            println!("display: Subscribing to topic '/faces'");
		            bsend.send(
		                Message::Subscribe(_aid,"/faces".to_string())
		            ).expect("display: failed to subscribe to topic '/faces'");
		        },

		        // display does shutdown
		        Message::Shutdown => {
		            println!("display: Shutdown");
		            // TODO do something 
		            break;
		        },

		        // display gets AppMessage
		        Message::App(topic,app_message) => {

		            match app_message {

		                // new face coordinates
		                AppMessage::FaceCoords => {
		                    println!("display: Received face coordinates from topic '{}'",topic);
		                },

		                // and nothing else
		                _ => { },
		            }
		        },

		        // and nothing else
		        _ => { },
		    }

		}

    });

    let mut window: PistonWindow = WindowSettings::new("Orbital", [1280,720]).exit_on_esc(true).graphics_api(OpenGL::V3_2).build().unwrap();
    let mut art: G2dTexture;

    art = Texture::from_path(&mut window.create_texture_context(),"snapshot.jpg",Flip::None,&TextureSettings::new()).unwrap();

    //window.set_lazy(true);

    while let Some(e) = window.next() {

	    art = Texture::from_path(&mut window.create_texture_context(),"snapshot.jpg",Flip::None,&TextureSettings::new()).unwrap();

        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            image(&art, c.transform, g);
        });

        while let Ok(cmd) = wrecv.try_recv() {
        	match cmd {
        		Drawable::Anything => {
        			println!("got a draw thing");
        		},
        		_ => { },
        	}
        }

    }



}
