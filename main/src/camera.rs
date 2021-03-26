

use std::time::Duration;
use std::thread;

//use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use appbroker::*;





///
/// Camera
/// This pretends to be a camera; publishing a pretend frame every second
///

pub fn app(_aid: AID, arecv: &Receiver<Message>, bsend: &Sender<Message>) {

	// TODO how bad is this to do?
	let arecv = arecv.clone();
	let bsend = bsend.clone();

	thread::spawn(move || {
   		loop {

		    thread::sleep(Duration::from_millis(1000));

		    // try get messages else fall through; should get and purge even if not used?
	        while let Ok(message) = arecv.try_recv() {

			    match message {

			        // camera does startup, initialize hardware, etc.
			        Message::Startup => {
			            println!("camera: Startup");
			        },

			        // camera does shutdown
			        Message::Shutdown => {
			            println!("camera: Shutdown");
			        },

			        // everything else camera doesn't care about
			        _ => { },
			    }

	        }

	        // uber hack
		    app_videocapture2();

            println!("camera: Force Tick");
            bsend.send(
                Message::App("/frames".to_string(),AppMessage::VideoFrame)
            ).expect("out_tx.send() failed!");



		}

   });

}






use std::io::BufRead;

use std::error::Error;

// there is some annoying namespace conflicts here - ignore for now
//use std::process::{Command, Stdio};
//use std::io::{BufRead, BufReader, Error, ErrorKind};

fn app_videocapture2() -> Result<(), std::io::Error> {
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




