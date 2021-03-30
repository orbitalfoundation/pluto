
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use crate::kernel::*;

pub struct Display {}
impl Display {
	pub fn new() -> Box<dyn Serviceable> {
		println!("Disiplay: creating");
		Box::new(Display {})
	}
}
impl Serviceable for Display {
    fn name(&self) -> &str { "Display" }
	fn stop(&self) {}
	fn start(&self, _sid: SID, _send: &Sender<Message>, _recv: &Receiver<Message> ) {
		let _send = _send.clone();
		let _recv = _recv.clone();
		let name = self.name();
		let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {
	        while let Ok(message) = _recv.recv() {
			    match message {
//			    	Message::Transmit()
			        _ => { },
			    }
	        }
		});
	}
}

/*

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

pub fn app(_aid: u32, arecv: &Receiver<Message>, bsend: &Sender<Message>)  {

println!("displ2");

	// a way to talk to windowing internal to this function
	// windowing has an issue where it cannot run in a subthread due to events
	let (wsend, wrecv) = unbounded::<Drawable>();
//	wsend.send(Drawable::Anything).unwrap();

println!("displ1");

	// TODO how bad is this to do?
	let arecv = arecv.clone();
	let bsend = bsend.clone();

    // start thread and look for messages
    thread::spawn(move || {

        // blocking wait for a message
        while let Ok(message) = arecv.recv() {

		    match message {

		        // display does shutdown
		        Message::Stop => {
		            println!("display: Shutdown");
		            // TODO do something 
		            break;
		        },


		        // display does startup, subscribes to face coordinate topic
		        Message::Start => {
		            println!("display: Startup");
		            println!("display: Subscribing to topic '/faces'");
		     //       bsend.send(
		       //         Message::Subscribe(_aid,"/faces".to_string())
		       //     ).expect("display: failed to subscribe to topic '/faces'");
		        },
/*
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
*/
		        // and nothing else
		        _ => { },
		    }

		}

    });

println!("displ12");

    // it is kind of * that this has to run on the main thread...
    // https://crates.io/crates/winit-main
    // https://www.jendrikillner.com/tags/match3/page/2/

    let mut window: PistonWindow = WindowSettings::new("Orbital", [1280,720]).exit_on_esc(true).graphics_api(OpenGL::V3_2).build().unwrap();

    window.set_lazy(true);

    //thread::spawn(move || {

	    let mut art: G2dTexture;
	    art = Texture::from_path(&mut window.create_texture_context(),"snapshot.jpg",Flip::None,&TextureSettings::new()).unwrap();

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

println!("display:done");
	//});

}

*/
