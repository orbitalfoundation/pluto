


use std::sync::mpsc::Sender;
use appmanager::*;



// The camera module reads camera hardware (here triggered by a timer)
fn camera_handler(_mid: MID,message: Message,out_tx: &Sender<Message>) {

    match message {

        // camera does startup, initialize hardware, etc.
        Message::Startup => {
            println!("camera: Startup");
        },

        // camera does shutdown
        Message::Shutdown => {
            println!("camera: Shutdown");
        },

        // camera receives a timer tick
        Message::Tick => {
            println!("camera: Tick");
            println!("camera: Reading the camera hardware and publishing the video frame.");
            out_tx.send(
                Message::App("/frames".to_string(),AppMessage::VideoFrame)
            ).expect("out_tx.send() failed!");
        },

        // everything else camera doesn't care about
        _ => { },
    }
}

fn face_detector_handler(mid: MID,message: Message,out_tx: &Sender<Message>) {
    
    match message {

        // face detector does startup, subscribe to the video topic
        Message::Startup => {
            println!("face_detector: Startup");
            println!("face_detector: Subscribing to topic '/frames'");
            out_tx.send(
                Message::Subscribe(mid,"/frames".to_string())
            ).expect("face_detector: failed to subscribe to topic '/frames'.");
        },

        // face detector does shutdown
        Message::Shutdown => {
            println!("face_detector: Shutdown");
        },

        // face detector gets an AppMessage
        Message::App(topic,app_message) => {

            match app_message {

                // face detector receives video frame
                AppMessage::VideoFrame => {
                    println!("face_detector: Received video frame from topic '{}'",topic);
                    println!("face_detector: Detecting face");
                    out_tx.send(
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


fn display_handler(mid: MID,message: Message,out_tx: &Sender<Message>) {
    
    match message {

        // display does startup, subscribes to face coordinate topic
        Message::Startup => {
            println!("display: Startup");
            println!("display: Subscribing to topic '/faces'");
            out_tx.send(
                Message::Subscribe(mid,"/faces".to_string())
            ).expect("display: failed to subscribe to topic '/faces'");
        },

        // display does shutdown
        Message::Shutdown => {
            println!("display: Shutdown");
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


fn main() {

	let mut manager = AppManager::new();

	manager.add(0,"Camera",display_handler);
	manager.add(1,"FaceDetector",display_handler);
	manager.add(2,"Display",display_handler);

	manager.run();

}

