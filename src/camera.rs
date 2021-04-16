/*

The below is a rust port of https://gist.github.com/bellbind/6954679 which is objc code for capture image from webcam

Most of what it does is send messages to objective c... this is the library it uses:

    https://github.com/SSheldon/rust-objc/blob/master/examples/example.rs

Also here is a bit of rust code that shows working with rust-objc messaging and cocoa:

    https://kyle.space/posts/cocoa-apps-in-rust-eventually/
    (Makepad itself also has good examples)

rust-media invents the idea of a core media buffer (although this code is quite old):

    https://github.com/pcwalton/rust-media/blob/master/platform/macos/coremedia.rs

this is a player that uses it

    https://github.com/pcwalton/rust-media/blob/master/platform/macos/videotoolbox.rs

here are some others

    https://github.com/LuoZijun/rust-core-media-sys
    https://github.com/LuoZijun/rust-core-video-sys/blob/master/src/lib.rs

notably 

    https://github.com/LuoZijun/rust-core-media-sys/blob/master/src/sample_buffer.rs

// links:
// some other library https://lib.rs/crates/objrs

*/

// ----------------------------------------------------------------------------------------------------
// This has to be first
#![allow(non_snake_case)]

// ----------------------------------------------------------------------------------------------------
// bind to some external test code

//#![allow(non_upper_case_globals)]
//#![allow(non_camel_case_types)]
//#![allow(non_snake_case)]
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[link(name = "avtest")]

// this just gets too overwhelming - i don't want to debug and fix a ton of header stuff
//include!("../avtest/avtestbind.in");


/*
// i cut and paste the contents of avtestbind.rs here and it no longer fails! sigh
extern "C" {
    pub fn avtest(
        device: AVCaptureDevice,
        input: AVCaptureDeviceInput,
        output: AVCaptureVideoDataOutput,
    );
}
*/

extern "C" {
    pub fn avtest(
        device: *mut Object,
        input: *mut Object,
        output: *mut Object,
    );
}

// ----------------------------------------------------------------------------------------------------
// get a few things
use core_foundation::base::{Boolean, CFRelease, CFRetain, CFTypeID, CFTypeRef, TCFType};

// ----------------------------------------------------------------------------------------------------
// I use AVFoundation, but I specify this in build.rs
// #[link(name = "AVFoundation", kind = "framework")]

// ----------------------------------------------------------------------------------------------------
// Objective C helper - does most of our bridging - does provide its own selector and sel! macros
use objc::runtime::{Class, Object, Sel, Protocol};
use objc::declare::ClassDecl;

// Macros annoyingly have to be specified in main.rs ... rust parser design that pollute scopes...
//#[macro_use] extern crate objc;

// ----------------------------------------------------------------------------------------------------
// NSString

// Can also make by hand:
//    let NSString = Class::get("NSString").unwrap();
use cocoa::foundation::NSString;

// various ways I can get at strings and manipulate them 
// use std::ffi::CString;
//let AVMediaTypeVideo = CString::new("vide").unwrap();
//let AVMediaTypeVideo = AVMediaTypeVideo.as_ptr(); // <- or I could use a StrongPtr:: from objc_rust
//let AVMediaTypeVideo: *mut Object = msg_send![class!(NSString), stringWithUTF8String:AVMediaTypeVideo];
//let AVMediaTypeVideo = NSString::alloc(nil).init_str(&"vide".to_string()).autorelease();

// Seems like I can get away with not releasing strings?
// use cocoa::foundation::NSAutoreleasePool;

// ----------------------------------------------------------------------------------------------------
// STANDALONE TEST SUPPORT
//use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular};

// ----------------------------------------------------------------------------------------------------
// NSLog ...
#[link(name = "CoreMedia", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "Foundation", kind = "framework")]
extern { pub fn NSLog(fmt: *mut Object, ...); }

// ----------------------------------------------------------------------------------------------------
// cocoa::base - provides a selector builder also
//#[allow(non_upper_case_globals)]
//type id = *mut Object;
//const nil: id = 0 as Id;
use cocoa::base::{SEL,selector, nil, id, NO, YES};

// ----------------------------------------------------------------------------------------------------
// get at type id - see https://github.com/pcwalton/rust-media/blob/master/platform/macos/coremedia.rs

#[link(name="CoreMedia", kind="framework")]
extern {
    pub fn CMSampleBufferGetTypeID() -> CFTypeID;
}

// ----------------------------------------------------------------------------------------------------
/// setup a camera and try start capturing frames
pub fn startav() {

    unsafe {

        // make secret enum type "vide" - not really documented anywhere but i did find a C# citation of this in a random reddit post...
        let AVMediaTypeVideo = NSString::alloc(nil).init_str(&"vide".to_string());

        // MAKE A DEVICE
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

/*
        // MAKE A DISPATCHER
        // This returns me a " OS_dispatch_queue_main: com.apple.main-thread "
        // https://developer.apple.com/documentation/dispatch/os_dispatch_queue_main
        // is that the right one? should i get one from winit?
        let queue = dispatch::ffi::dispatch_get_main_queue();
        NSLog(NSString::alloc(nil).init_str("queue is %@"),queue);

        // MAKE A CAPTURE HANDLER
        let mut Capture = ClassDecl::new("MyCapture", class!(NSObject)).unwrap();

        // Throw in the protocol thing - not sure if it matters - I've heard it does not matter?
        // See protocol spec at https://developer.apple.com/documentation/avfoundation/avcapturevideodataoutputsamplebufferdelegate
        // Also see https://developer.apple.com/documentation/avfoundation/avcapturevideodataoutput/1389008-setsamplebufferdelegate
        // NOTE -> protocol decl is ignored no diff - for example i can comment this next line out or not - makes no diff
        Capture.add_protocol(&Protocol::get("AVCaptureVideoDataOutputSampleBufferDelegate").unwrap());

        // Build a capture output handler with the right signature
        // This is the signature:
        //      https://developer.apple.com/documentation/avfoundation/avcapturevideodataoutputsamplebufferdelegate
        //
        // Apple says it should look like this:
        //      func captureOutput(AVCaptureOutput, didOutput: CMSampleBuffer, from: AVCaptureConnection)
        //
        // The objective-C implementation that does work looks like this:
        //
        //      - (void)       captureOutput: (AVCaptureOutput*) output
        //             didOutputSampleBuffer: (CMSampleBufferRef) buffer
        //                    fromConnection: (AVCaptureConnection*) connection 
        //
        // Apparently when you make a selector it mashes up all these fields to make a signature.... and rust_objc is just a macro around cocoas
        //
        //            sel!(captureOutput: didOutput: from:)
        //
        // ? I'm still not sure why there are colons on these ? it is idiosyncratic compared to other selectors.
        // NOTE we can also switch to using the cocoa selector builder instead if we want - we don't have to use the one from rust_objc
        // https://github.com/SSheldon/rust-objc/blob/master/src/macros.rs
        //
        // In general - what is signature production?
        //    https://developer.apple.com/library/archive/documentation/General/Conceptual/DevPedia-CocoaCore/Selector.html
        //
        // So I think it will be something like "captureOutput:didOutput:from:"
        //

        //
        // Also - separately - we may need the right signature for the class type???
        //   https://developer.apple.com/documentation/coremedia/1489662-cmsamplebuffergettypeid
        //   

        let typeid: CFTypeID = CMSampleBufferGetTypeID();

        extern fn myCaptureOutput(_this: &Object, _cmd: Sel, id1: id, id2: id, id3: id) {
            println!("stuff");
        }

        // paranoia check
        let magic1 = sel!(captureOutput: didOutput: from:);
        let magic2 = selector("captureOutput:didOutput:from:");
        if(magic1 == magic2) { println!("they are similar as expected"); }

        // NOTE -> even the wrong name here does not report an issue

        Capture.add_method(magic2, myCaptureOutput as extern fn(&Object,Sel, id, id, id));

        Capture.register();
        let Capture = Class::get("MyCapture").unwrap(); // why can't I somehow dereference the one I built above?
        let capture: *mut Object = msg_send![Capture,alloc];
        let capture: *mut Object = msg_send![capture,init];
        NSLog(NSString::alloc(nil).init_str("Capture is %@"),capture);
        // The goal is to mimic this piece of objective-c code: [output setSampleBufferDelegate: capture queue: dispatch_get_main_queue()];
// NOTE -> I can pass a nil object - it makes no diff to what happens.... or i can pass say "input" or any other random object and has no impact -> tells me the delegate is not being invoked 
        let _: () = msg_send![output, setSampleBufferDelegate:capture queue:queue];
 */

avtest(device,input,output);


        // MAKE SESSION AND START IT

        let session: *mut Object = msg_send![class!(AVCaptureSession),alloc];
        let session: *mut Object = msg_send![session,init];
        let _: () = msg_send![session,addInput:input];
        let _: () = msg_send![session,addOutput:output];
        let _: () = msg_send![session,startRunning];
        NSLog(NSString::alloc(nil).init_str("Session is %@"),session);

        // see if anything happens
        std::thread::sleep(std::time::Duration::from_millis(1000));

        // DISPATCHERS???
        // https://github.com/SSheldon/rust-dispatch/blob/master/examples/main.rs
        // https://faq.sealedabstract.com/rust/#dispatch
        //dispatch::ffi::dispatch_main();

        // STANDALONE?
        // TEST STANDALONE
        //let _pool = NSAutoreleasePool::new(nil);
        //let app = NSApp();
        //app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        //app.run();

        // this does not work... if i let things fall through to makepad we are fine however
        //unsafe { core_foundation::runloop::CFRunLoopRun(); }

   }

    println!("falling out to the rest of the system");

}



/*

solutions?

x is dispatcher borked? -> seems like in other examples the callback will be hit once
x is callback running but println is piped away? no
- is callback selector bad?

- make my own dispatcher ?  DispatchQueue(label: "myqueue") is this a dispatch issue? does not seem to be... should at least fire once

- make it in c - and print it here?

- find other examples of making in c?

- selector is wrong? try catch all selectors better? here are examples of how others are trying:

    func captureOutput(captureOutput: AVCaptureOutput!, didDropSampleBuffer sampleBuffer: CMSampleBuffer!, fromConnection connection: AVCaptureConnection!) {
    func captureOutput(captureOutput: AVCaptureOutput!, didOutputSampleBuffer sampleBuffer: CMSampleBuffer!, fromConnection connection: AVCaptureConnection!) {
    func captureOutput(_ output: AVCaptureOutput, didOutput sampleBuffer: CMSampleBuffer, from connection: AVCaptureConnection) {
  -      captureOutput:(AVCaptureOutput *)captureOutput didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer fromConnection:(AVCaptureConnection *)connection {
- (void)captureOutput:(AVCaptureOutput *)captureOutput didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer fromConnection:(AVCaptureConnection *)connection {
I found that that captureOutput:didOutputSampleBuffer:fromConnection is NOT called and I would like to know why or what I am doing wrong.
- (void)captureOutput:(AVCaptureOutput *)captureOutput didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer fromConnection:(AVCaptureConnection *)connection
 func captureOutput(captureOutput: AVCaptureOutput, didOutputSampleBuffer sampleBuffer: CMSampleBufferRef, fromConnection connection: AVCaptureConnection) {

didOutputSampleBuffer? -> is this useful?

*/

























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























