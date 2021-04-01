
use crossbeam::channel::*;
use crate::kernel::*;

#[derive(Clone)]
pub struct Display {
}
impl Display {
	pub fn new() -> Box<dyn Serviceable> {
		Box::new(Self {})
	}
}
impl Serviceable for Display {
    fn name(&self) -> &str { "Display" }
	fn stop(&self) {}
	fn start(&self, sid: SID, send: &Sender<Message>, recv: &Receiver<Message> ) {

		// hack - there is a race condition because the registration of this handler is in a sub-thread
	    std::thread::sleep(std::time::Duration::from_millis(1000));

		// listen to display messages
		//let send = send.clone();
		//let recv = recv.clone();
		send.send(
		    Message::Subscribe(sid,"/display".to_string())
		).expect("Display: failed to subscribe");

		// run on main thread - never return
		winit_greedy_main_thread(&recv);

	}
}

// the goal for now is to produce any view; so it doesn't matter what display engine is used
extern crate piston_window;
use piston_window::*;

// I was thinking of having a huge list of kinds of drawables for now?
//pub enum Drawable {
//	Anything,
//	Circle(u32),
//	Box(u32,u32,u32,u32),
//}

pub fn winit_greedy_main_thread(recv:&Receiver<Message>) {

    let mut window: PistonWindow = WindowSettings::new("Orbital", [1280,720]).exit_on_esc(true).graphics_api(OpenGL::V3_2).build().unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {

	    let art: G2dTexture = Texture::from_path(&mut window.create_texture_context(),"snapshot.jpg",Flip::None,&TextureSettings::new()).unwrap();

        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            image(&art, c.transform, g);
        });

        while let Ok(message) = recv.try_recv() {
        	println!("Display: got message");
        	match message {
		    	Message::Event(topic,data) => {
		    		println!("Display: got message {} {}",topic, data);
		    	},
        		_ => { },
        	}
        }
    }
}

