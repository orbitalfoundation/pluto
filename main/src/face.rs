


//use std::time::Duration;
use std::thread;

//use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use appbroker::*;


///
/// Pretend Face Detector
///

pub fn app(_aid: AID, arecv: &Receiver<Message>, bsend: &Sender<Message>)  {

	// TODO how bad is this to do?
	let arecv = arecv.clone();
	let bsend = bsend.clone();

	// TODO - 1) arguably this could chose to bind directly to the camera device conceptually
	//      - 2) also, does it make sense to wait for a startup message or just try bind now?
	//      - 3) also every package should probably list their own startup dependencies

    println!("face_detector: Subscribing to topic '/frames'");
    bsend.send(
        Message::Subscribe(_aid,"/frames".to_string())
    ).expect("face_detector: failed to subscribe to topic '/frames'.");

    // start thread and look for messages
    thread::spawn(move || {

        // blocking wait forever
        while let Ok(message) = arecv.recv() {
    
		    match message {

		        Message::App(topic,app_message) => {

		            match app_message {

		                // face detector receives video frame
		                AppMessage::VideoFrame => {
		                    println!("face_detector: Received video frame from topic '{}'",topic);
		                    bsend.send(
		                        Message::App("/faces".to_string(),AppMessage::FaceCoords)
		                    ).expect("face_detector: failed to send video frame!");
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

}
