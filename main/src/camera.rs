
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
	fn start(&self, _sid: SID, send: &Sender<Message>, recv: &Receiver<Message> ) {
		let send = send.clone();
		let recv = recv.clone();
		let name = self.name();
		let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {
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
