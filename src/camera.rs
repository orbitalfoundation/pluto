
use crossbeam::channel::*;
use crate::kernel::*;

#[derive(Clone)]
pub struct Camera {}
impl Camera {
	pub fn new() -> Box<dyn Serviceable> {
		Box::new(Self {})
	}
}
impl Serviceable for Camera {
    fn name(&self) -> &str { "Camera" }
	fn stop(&self) {}
	fn start(&self, _name: String, _sid: SID, send: Sender<Message>, recv: Receiver<Message> ) {
		let send = send.clone();
		let recv = recv.clone();
		let name = self.name();
		let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {

			// for now wait for an order to deliver a frame
		    send.send(Message::Subscribe(_sid,"/camera".to_string())).expect("Camera: failed to subscribe");

		    // emit a frame on command only
		    // obviously this is all pretend - no real frame is sent here
		    // TODO send back to caller only
		    // TODO use a shared memory buffer pattern
		    // TODO can start sending until ordered to stop
	        while let Ok(message) = recv.recv() {
			    match message {
			    	Message::Event(topic,data) => {
			    		println!("Camera: Received: {} {}",topic, data);
			    		let message = Message::Event("/frames".to_string(),"[A FRAME OF VIDEO]".to_string());
						send.send(message).expect("error");
			    	},
			        _ => { },
			    }
	        }

		    /* not used - in this pattern frames were proactively published; in this test I was use using the command line to fetch real frames
			loop {
			    std::thread::sleep(std::time::Duration::from_millis(1000));
		        while let Ok(message) = recv.try_recv() {
				    match message {
				        _ => { },
				    }
		        }

			    let _results = app_videocapture2();

				send.send(
					Message::Event("/frames".to_string(),"AMAZING!".to_string())
				).expect("send.send() failed!");

			}
			*/
		});
	}
}

use std::io::BufRead;
//use std::error::Error;

// there is some namespace conflicts here - ignore for now
//use std::process::{Command, Stdio};
//use std::io::{BufRead, BufReader, Error, ErrorKind};

fn app_videocapture2() -> Result<(), std::io::Error> {
    println!("camera: capturing a frame");
    let stdout = std::process::Command::new("/opt/homebrew/bin/imagesnap")
        .stdout(std::process::Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other,"Could not capture standard output."))?;

    let reader = std::io::BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

     Ok(())
}
