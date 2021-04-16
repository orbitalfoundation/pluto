
//
// Rust WebCam access using AVFoundation - see these useful and fun links:
//
// https://gist.github.com/bellbind/6954679
// https://github.com/SSheldon/rust-objc/blob/master/examples/example.rs
// https://kyle.space/posts/cocoa-apps-in-rust-eventually/
// https://github.com/pcwalton/rust-media/blob/master/platform/macos/coremedia.rs
//

// ----------------------------------------------------------------------------------------------------
// This has to be first
#![allow(non_snake_case)]

// ----------------------------------------------------------------------------------------------------
// bind to an objective c native layer to perform some avfoundation operations - no longer used

//#![allow(non_upper_case_globals)]
//#![allow(non_camel_case_types)]
//#![allow(non_snake_case)]
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// #[link(name = "avtest")]

// ----------------------------------------------------------------------------------------------------
// pull in all of avfoundation headers as built by bindgen - no longer used
// this is not used because products are huge and buggy - it's just too overwhelming to deal with
//include!("../avtest/avtestbind.in");

// ----------------------------------------------------------------------------------------------------
// super weird bug work around - no longer used
// if I include a source file with the below then there are compile time errors - avtest not foudn
// if I cut and paste that here - then those errors go away... 
// perhaps i don't understand how include works
//

// avfoundation bindgen version with full type declarations
//extern "C" {
//    pub fn avtest(
//        device: AVCaptureDevice,
//        input: AVCaptureDeviceInput,
//        output: AVCaptureVideoDataOutput,
//    );
//}

// void* version which is easier to build and does not rely on buggy bindgen attempt at avfoundation
//extern "C" {
//    pub fn avtest(
//        device: *mut Object,
//        input: *mut Object,
//        output: *mut Object,
//    );
//}

// ----------------------------------------------------------------------------------------------------
// Objective C helper - does most of our bridging - does provide its own selector and sel! macros
use objc::runtime::{Class, Object, Sel, Protocol};
use objc::declare::ClassDecl;

// Macros annoyingly have to be specified in main.rs ... bad rust parser design that pollute scopes...
//#[macro_use] extern crate objc;

// ----------------------------------------------------------------------------------------------------
//  get services from core foundation
//use core_foundation::base::{CFTypeID};

// ----------------------------------------------------------------------------------------------------
// build.rs can specify these also... notably the app will link but will fail to run without these
#[link(name = "AVFoundation", kind = "framework")]
//#[link(name = "CoreMedia", kind = "framework")]
//#[link(name = "CoreFoundation", kind = "framework")]
//#[link(name = "Foundation", kind = "framework")]
extern { pub fn NSLog(fmt: *mut Object, ...); }

// ----------------------------------------------------------------------------------------------------
// NSString

use cocoa::foundation::NSString;

// various ways I can get at strings and manipulate them 
// use std::ffi::CString;
// CString::new("vide").unwrap();
// msg_send![class!(NSString), stringWithUTF8String:AVMediaTypeVideo];
// NSString::alloc(nil).init_str(&"something".to_string()).autorelease();
// let NSString = Class::get("NSString").unwrap();
// Seems like I can get away with not releasing strings?
// use cocoa::foundation::NSAutoreleasePool;

// ----------------------------------------------------------------------------------------------------
// cocoa::base - provides a selector builder also
//#[allow(non_upper_case_globals)]
//type id = *mut Object;
//const nil: id = 0 as Id;
use cocoa::base::{nil, id};

// ----------------------------------------------------------------------------------------------------
/// setup a camera and try start capturing frames
pub fn startav() {

    unsafe {

        // MAKE A DEVICE
        let AVMediaTypeVideo = NSString::alloc(nil).init_str(&"vide".to_string());
        let device: *mut Object = msg_send![class!(AVCaptureDevice), defaultDeviceWithMediaType:AVMediaTypeVideo ];
        NSLog(NSString::alloc(nil).init_str("Device is %@"),device);

        // MAKE AN INPUT
        let input: *mut Object = msg_send![class!(AVCaptureDeviceInput), deviceInputWithDevice:device error:0 ]; 
        NSLog(NSString::alloc(nil).init_str("Input is %@"),input);

        // MAKE AN OUTPUT
        let output: *mut Object = msg_send![class!(AVCaptureVideoDataOutput),alloc];
        let output: *mut Object = msg_send![output,init];
        //let _: () = msg_send![output,alwaysDiscardsLateVideoFrames:YES];
        //let _: () = msg_send![output,setEnabled:YES]; [[output connectionWithMediaType:AVMediaTypeVideo] setEnabled:YES];

        // MAKE A DISPATCHER
        let queue = dispatch::ffi::dispatch_get_main_queue();
        NSLog(NSString::alloc(nil).init_str("queue is %@"),queue);

        // MAKE CALLBACK
        extern fn myCaptureOutput(_this: &Object, _cmd: Sel, _id1: id, _id2: id, _id3: id) {
            // https://developer.apple.com/documentation/coremedia/1489662-cmsamplebuffergettypeid
            // let typeid: CFTypeID = CMSampleBufferGetTypeID();
            println!("stuff");
        }

        // MAKE A CAPTURE HANDLER
        let mut Capture = ClassDecl::new("MyCapture", class!(NSObject)).unwrap();
        let protocol = &Protocol::get("AVCaptureVideoDataOutputSampleBufferDelegate").unwrap();
        Capture.add_protocol(protocol);
        let magic = sel!(captureOutput: didOutputSampleBuffer: fromConnection:);
        Capture.add_method(magic, myCaptureOutput as extern fn(&Object,Sel, id, id, id));
        Capture.register();
        let Capture = Class::get("MyCapture").unwrap(); // why can't I somehow dereference the one I built above?
        let capture: *mut Object = msg_send![Capture,alloc];
        let capture: *mut Object = msg_send![capture,init];
        NSLog(NSString::alloc(nil).init_str("Capture is %@"),capture);
        let _: () = msg_send![output, setSampleBufferDelegate:capture queue:queue];

        // MAKE SESSION
        let session: *mut Object = msg_send![class!(AVCaptureSession),alloc];
        let session: *mut Object = msg_send![session,init];
        let _: () = msg_send![session,addInput:input];
        let _: () = msg_send![session,addOutput:output];
        let _: () = msg_send![session,startRunning];
        NSLog(NSString::alloc(nil).init_str("Session is %@"),session);
   }

    println!("falling out to the rest of the system");

}



/////////////////////////////////////////////////////////////////////////////////////////
// connect to orbital microkernel
/////////////////////////////////////////////////////////////////////////////////////////

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

// hard start video capture as a test
startav();

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



/* super hack - just shell out to get a frame

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

*/























