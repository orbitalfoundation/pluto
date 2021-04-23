
// This has to be first
#![allow(non_snake_case)]


/////////////////////////////////////////////////////////////////////////////////////////
// a singleton 
/////////////////////////////////////////////////////////////////////////////////////////

use std::sync::{Once};
use std::time::Duration;
use std::{mem};

#[derive(Clone)]
struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Arc<Mutex<u8>>,
    memory: Arc<Mutex<Box<[u32;262144]>>>,
}

fn singleton() -> SingletonReader {

    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {

            const SIZE: usize = 512*512;
            let mut memory: Box<[u32;SIZE]> = Box::new([0;SIZE]);
            let mut sharedmemory = Arc::new(Mutex::new(memory));

            let singleton = SingletonReader {
                inner: Arc::new(Mutex::new(0)),
                memory: sharedmemory,
            };

            SINGLETON = mem::transmute(Box::new(singleton));
        });

        (*SINGLETON).clone()
    }
}


/////////////////////////////////////////////////////////////////////////////////////////
// connect to orbital microkernel
/////////////////////////////////////////////////////////////////////////////////////////

//
// what is the best pattern for sharing memory?
//
// how do we allocate some ram?
//      1) we can make structured memory like "vec" but that seems overkill... we don't need those capabilities, we just want some RAM
//      2) vec itself does something like this which seems overkill as well https://doc.rust-lang.org/nomicon/vec-alloc.html
//      3) rust has a concept of a "Box"? https://doc.rust-lang.org/nomicon/vec-alloc.html -> but what is the "TYPE" of this memory?
//      4) another idea is something like Box<[u8;32]> <- see https://users.rust-lang.org/t/how-to-malloc-an-array-in-heap-like-c/27827/23
//      5) another idea is just use libc! let arr = libc::malloc(std::mem::size_of::<i32>() * array_size) as *mut i32;
//      6) another idea is something like  let vec = Vec::with_capacity(size); let arr = vec.as_ptr();
//      7) here is a crate https://docs.rs/heaparray/0.4.3/heaparray/
//      8) (box is a thing that means put onto the heap https://doc.rust-lang.org/beta/rust-by-example/std/box.html )
//      9) a pure square bracket thing is a way to declare a raw array [u32;size] -> a raw array -> see https://doc.rust-lang.org/std/primitive.array.html
//
// how do we make this thread safe?
//      1) https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/
//      2) clearly i share a pointer or reference, there is a pattern Arc::new(Mutex::new(thing)) -> is this going to then be/send just a pointer?
//      3) https://doc.rust-lang.org/stable/nomicon/send-and-sync.html
//

use crossbeam::channel::*;
use crate::kernel::*;

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

static mut mymemory: u32 = 12;

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

        // START VIDEO RECEIVER
        appleWebCamCaptureStart();

        // start a separate thread to watch for commands
        let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {

            // wait till display is up
            std::thread::sleep(std::time::Duration::from_millis(2000));

            loop {
   
                // send.send(Message::Subscribe(_sid,"/camera".to_string())).expect("Camera: failed to subscribe");
                while let Ok(message) = recv.try_recv() {
                    match message {
                        Message::Event(topic,data) => {
                            println!("Camera: Received: {} {}",topic, data);
                            let message = Message::Event("/frames".to_string(),"[A FRAME OF VIDEO]".to_string());
                            send.send(message).expect("error");
                        },
                        _ => { },
                    }
                }

                // this is done in this thread for now because send is not visible to camera yet
                // wait so as not to thrash
                std::thread::sleep(std::time::Duration::from_millis(100));
                let mut memory = singleton().memory;
                let messagetosend = Message::Share(memory.clone());
                send.send(messagetosend).expect("error");

            }

        });
    }
}


/////////////////////////////////////////////////////////////////////////////////////////
// test code throwaway
/////////////////////////////////////////////////////////////////////////////////////////

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




/////////////////////////////////////////////////////////////////////////////////////////
// get at apple avfoundation webcam
/////////////////////////////////////////////////////////////////////////////////////////


//
// Rust WebCam access using AVFoundation - see these useful and fun links:
//
// https://gist.github.com/bellbind/6954679
// https://github.com/SSheldon/rust-objc/blob/master/examples/example.rs
// https://kyle.space/posts/cocoa-apps-in-rust-eventually/
// https://github.com/pcwalton/rust-media/blob/master/platform/macos/coremedia.rs
//



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
#[link(name = "CoreMedia", kind = "framework")]
#[link(name = "CoreImage", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "Foundation", kind = "framework")]
extern { pub fn NSLog(fmt: *mut Object, ...); }

// ----------------------------------------------------------------------------------------------------
// NSString

use cocoa::foundation::NSString;
use cocoa::appkit::NSColor;

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
use cocoa::base::{nil, id, NO, YES};

// ----------------------------------------------------------------------------------------------------
// trying to get at some of these methods; seems easiest to just use id

/*
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVBuffer {
    _unused: [u8; 0],
}
pub type CVBufferRef = *mut __CVBuffer;
pub type CVImageBufferRef = CVBufferRef;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct opaqueCMSampleBuffer {
    _unused: [u8; 0],
}
pub type CMSampleBufferRef = *mut opaqueCMSampleBuffer;
*/

extern "C" {
    pub fn CMSampleBufferGetImageBuffer(buffer: id) -> id;
    pub fn CVPixelBufferGetBaseAddress(buffer:id) -> id;
    pub fn CVPixelBufferLockBaseAddress(buffer:id,flags:u64);
    pub fn CVPixelBufferUnlockBaseAddress(buffer:id,flags:u64);
    pub fn CVPixelBufferGetWidth(buffer:id) -> u64;
    pub fn CVPixelBufferGetHeight(buffer:id) -> u64;
    pub fn CVPixelBufferGetBaseAddressOfPlane(buffer:id,flags:u64) -> id;
    // pub fn CMSampleBufferGetOutputPresentationTimeStamp(buffer:id) -> id;
    // pub fn CMTimeGetSeconds(time:id) -> f64;
}

generate_counter!(Counter, usize);


extern fn appleWebCamCaptureOutput(_this: &Object, _cmd: Sel, _id1: id, sbuf: id, _id3: id) {

    unsafe {

        // get timestamp - this crashes...
        //let time = CMSampleBufferGetOutputPresentationTimeStamp(buffer);
        //let time = CMTimeGetSeconds(time);
        //println!("Time is {}",time);

        // given a CMSampleBuffer convert to a CVImageBuffer and also is a CVPixelBuffer
        let ibuf = CMSampleBufferGetImageBuffer(sbuf);

        // given a CVImageBuffer return a CIImage (this may have to be done before any more operations on ibuf)
        let image: *mut Object = msg_send![class!(CIImage), imageWithCVImageBuffer: ibuf];

        /* this prints this:
         <CVPixelBuffer 0x10fc80050 width=1280 height=720 bytesPerRow=2560 pixelFormat=2vuy iosurface=0x11f078270 attributes={
            Height = 720;
            IOSurfaceProperties =     {
                IOSurfacePurgeWhenNotInUse = 1;
            };
            PixelFormatType = 846624121;
            Width = 1280;
        } propagatedAttachments={
            CVImageBufferColorPrimaries = "ITU_R_709_2";
            CVImageBufferTransferFunction = "ITU_R_709_2";
            CVImageBufferYCbCrMatrix = "ITU_R_601_4";
        } nonPropagatedAttachments={
        }>
        */
        //NSLog(NSString::alloc(nil).init_str("ImageBuf is %@"),ibuf);

        // this prints this: <CIImage: 0x11df090f0 extent [0 0 1280 720]>
        //NSLog(NSString::alloc(nil).init_str("CIImage is %@"),image);

        // GET AT PIXELS ATTEMPT #1: TRY LOCK ADDRESS AND PEEK
        //
        // this crashes or returns null if I do not lock it
        //CVPixelBufferLockBaseAddress(ibuf, 0);
        //let baseAddress: id = CVPixelBufferGetBaseAddress(ibuf);
        //NSLog(NSString::alloc(nil).init_str("DATA is %@"),baseAddress);
        //CVPixelBufferUnlockBaseAddress(ibuf,0);

        // also crashes
        //let lumaBaseAddress = CVPixelBufferGetBaseAddressOfPlane(ibuf, 0);
        //NSLog(NSString::alloc(nil).init_str("DATA is %@"),lumaBaseAddress);

        // if i could get at a raw buffer then I could browse it...
        //    let ptr = baseAddress as *mut u32;
        //    let val = *(ptr.add(1));
        //    println!("peering at raw buffer {}",val);

        // some queries work...
        let width = CVPixelBufferGetWidth(ibuf);
        let height = CVPixelBufferGetHeight(ibuf);

        // given a CIImage return an NSBitmapImageRep and populate it
        let bitmap: *mut Object = msg_send![class!(NSBitmapImageRep), alloc];
        let _: () = msg_send![bitmap,initWithCIImage: image];
        NSLog(NSString::alloc(nil).init_str("DATA is %@"),bitmap);

        //this works
        let w: u64 = msg_send![bitmap,pixelsWide];
        let h: u64 = msg_send![bitmap,pixelsHigh];
        let m: u64 = msg_send![bitmap,bytesPerRow];
        let w = w as usize;
        let h = h as usize;
        let m = m as usize;
        let raw: *mut u32 = msg_send![bitmap,bitmapData];

        // how long is this taking?
        //use std::time::Instant;
        //let now = Instant::now();

        // write to the raw pixels
        let memory = singleton().memory;
        let mut ptr = memory.lock().unwrap();
        for y in 0..512{
            for x in 0..512{

                // GET AT PIXELS ATTEMPT #2: GET A POINTER

                let pixel = *(raw.add(y*w+x));

                ptr[y*512+x]=pixel.swap_bytes().rotate_right(8);  // target format is ARGB ignoring A, and src format is probaby RGBA

                /*
                // GET AT PIXELS ATTEMPT #3: CONVERT EACH ONE TO NSColor tediously -> this works but it is so slow it silently fails because it runs out of time

                // get one pixel as an NSColor -> this works and returns a NSDeviceRGBColorSpace triplet
                let cspace: *mut Object = msg_send![bitmap, colorAtX:x y:y];
                //NSLog(NSString::alloc(nil).init_str("COLOR is %@"),cspace);

                // ?can i cast this to become rust visible NSColor? no - because NSColor is a trait and Rust is unable to cast a reference to a trait absurdly
                // https://stackoverflow.com/questions/34419561/can-i-cast-between-two-traits
                // http://idubrov.name/rust/2018/06/16/dynamic-casting-traits.html
                //unsafe {
                //let testColor = cspace as *NSColor;
                //let testcolor = NSColor::colorWithRed_green_blue_alpha_(nil, 0.5, 0.3, 0.9, 1.0);
                //println!("Test color is {}",testcolor.blueComponent());
                //}

                // try get one color from this in turn - this fails to extract the color - it just returns the whole blob again
                let r: f64 = msg_send![cspace, redComponent];
                let g: f64 = msg_send![cspace, greenComponent];
                let b: f64 = msg_send![cspace, blueComponent];

                let r = (r*255.0) as u32;
                let g = (g*255.0) as u32;
                let b = (b*255.0) as u32;

                let c = r*65536 + g*256 + b;

                ptr[y*512+x]=c;
                */
            }
        }

        // build a png
        if false {
            let filename = format!("result{}.png",Counter::next() );
            let filename = NSString::alloc(nil).init_str(filename.as_str());
            let data: *mut Object = msg_send![bitmap, representationUsingType:4 properties: nil];
            let _: () = msg_send![data, writeToFile: filename atomically: YES];
        }

        //let elapsed = now.elapsed();
        //println!("The Camera paint routine took {:.2?}",elapsed);
    }

}



fn appleWebCamCaptureStart() {
    unsafe {

        let myc = |_this: &Object, _cmd: Sel, _id1: id, buffer: id, _id3: id | {
            println!("got video");
        };

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

        // MAKE A CAPTURE HANDLER
        let mut Capture = ClassDecl::new("MyCapture", class!(NSObject)).unwrap();
        let protocol = &Protocol::get("AVCaptureVideoDataOutputSampleBufferDelegate").unwrap();
        Capture.add_protocol(protocol);
        let magic = sel!(captureOutput: didOutputSampleBuffer: fromConnection:);
        Capture.add_method(magic, appleWebCamCaptureOutput as extern fn(&Object,Sel, id, id, id));
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
}


