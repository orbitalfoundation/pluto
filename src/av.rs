
// the below is a rust port of https://gist.github.com/bellbind/6954679
// capture image from webcam(e.g. face time)

// some helpful examples
// https://kyle.space/posts/cocoa-apps-in-rust-eventually/

// msg_send
// https://github.com/SSheldon/rust-objc/blob/master/examples/example.rs

// silence a few warnings
#![allow(non_snake_case)]

// don't need to specify these by hand...
//extern crate dispatch;
//extern crate cocoa;

// do get macros
#[macro_use] extern crate objc;

use std::ffi::CString;

// not even sure this is will work - is this the right dispatcher?
use dispatch::{Queue,QueueAttribute};

use cocoa::base::{SEL,selector, nil, id, NO, YES};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSBackingStoreBuffered, NSMenu, NSMenuItem, NSWindowStyleMask,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps};
use objc::runtime::{Class, Object, Sel, BOOL, Protocol};
use objc::declare::ClassDecl;
use objc::Encode;
use objc::rc::StrongPtr;

// can't seem to figure out how to get nslog ... the author of objc could have been more loquacious
//use objc::NSLog; ???
#[link(name = "Foundation", kind = "framework")]
extern {
    // objc
    //pub fn objc_getClass(name: *const libc::c_char) -> Class;
    //pub fn objc_msgSend(obj: id, sel: SEL, ...) -> id;
    //pub fn sel_registerName(name: *const libc::c_char) -> SEL;

    // Foundation
    pub fn NSLog(fmt: id, ...);
    //pub fn NSStringFromClass(cls: Class) -> id;
    //pub fn NSStringFromSelector(sel: SEL) -> id;
}


/// startup
pub fn startav() {

    unsafe {

        // some types
        let NSString = Class::get("NSString").unwrap();

        // make secret enum type "vide" - not really documented anywhere but i did find a C# citation of this
        //let AVMediaTypeVideo = CString::new("vide").unwrap();
        //let AVMediaTypeVideo = AVMediaTypeVideo.as_ptr();
        //let AVMediaTypeVideo: *mut Object = msg_send![class!(NSString), stringWithUTF8String:AVMediaTypeVideo];
        let AVMediaTypeVideo = NSString::alloc(nil).init_str(&"vide".to_string()).autorelease();

        // make the device
        let device: *mut Object = msg_send![class!(AVCaptureDevice), defaultDeviceWithMediaType:AVMediaTypeVideo ];
        NSLog(NSString::alloc(nil).init_str("Device is %@"),device);

        // make the input
        let input: *mut Object = msg_send![class!(AVCaptureDeviceInput), deviceInputWithDevice:device error:0 ]; 
        NSLog(NSString::alloc(nil).init_str("Input is %@"),input);

        // make the output thing
        let output: *mut Object = msg_send![class!(AVCaptureVideoDataOutput),alloc];
        let output: *mut Object = msg_send![output,init];

        // make a capture class
        // how do i implement AVCaptureVideoDataOutputSampleBufferDelegate?
        // is that a polymorphic method?
        // https://developer.apple.com/documentation/avfoundation/avcapturevideodataoutputsamplebufferdelegate
        let mut Capture = ClassDecl::new("MyCapture", class!(NSObject)).unwrap();
        extern fn myCaptureOutput(_this: &Object, _cmd: Sel) {
            println!("stuff");
            // is this being reached?
        }
        Capture.add_method(sel!(captureOutput), myCaptureOutput as extern fn(&Object,Sel));
        Capture.register();
        let Capture = Class::get("MyCapture").unwrap(); // why?
        let capture: *mut Object = msg_send![Capture,alloc];
        let capture: *mut Object = msg_send![capture,init];
        NSLog(NSString::alloc(nil).init_str("Capture is %@"),capture);

        // how msg_send works and its notation could have been clarified more...
        // 2 params? https://developer.apple.com/documentation/avfoundation/avcapturevideodataoutput/1389008-setsamplebufferdelegate
        // or ??   [output setSampleBufferDelegate: capture queue: dispatch_get_main_queue()];
        // also - no commas on msg_send?
        // is there a non macro version?

        // make some kind of dispatch?
        let queue = dispatch::ffi::dispatch_get_main_queue();
        NSLog(NSString::alloc(nil).init_str("queue is %@"),queue);

        //let _: () = msg_send![output, sampleBufferDelegate:capture sampleBufferCallBackQueue:queue];
        let _: () = msg_send![output, setSampleBufferDelegate:capture queue:queue];

        // make and start the session itself
        let session: *mut Object = msg_send![class!(AVCaptureSession),alloc];
        let session: *mut Object = msg_send![session,init];
        let _: () = msg_send![session,addInput:input];
        let _: () = msg_send![session,addOutput:output];
        let _: () = msg_send![session,startRunning];
        NSLog(NSString::alloc(nil).init_str("Session is %@"),session);

        // TODO - i think i somehow need to let this thing now fire events
        // right now my capture class is not being invoked (i think)
        // is that because it is not of type AVCaptureVideoDataOutputSampleBufferDelegate
        // or is it missing a selector?
        // or is that the dispatcher needs to run?

        // how does dispatch work? what does it mean?
        // https://github.com/SSheldon/rust-dispatch/blob/master/examples/main.rs
        // https://faq.sealedabstract.com/rust/#dispatch
//        dispatch::ffi::dispatch_main();

        // this crashes
        //let app = NSRunningApplication::currentApplication(nil);
        //app.run();

        // maybe this?
        println!("running");
        core_foundation::runloop::CFRunLoopRun();

        // hmm
        //std::thread::sleep(std::time::Duration::from_millis(3000));

   }


}

// links:
// some other library https://lib.rs/crates/objrs
























fn oldmain() {

    fn register_button() {
        unsafe {
            let superclass = class!(NSButton);
            let mut decl = ClassDecl::new("HelloWorldButton", superclass).unwrap();

            extern fn clicked(_this: &Object, _cmd: Sel) {
                unsafe {
                    println!("clicked {:?}", _this); 

/*
                     let alert:*const Object = msg_send!(class!(NSAlert), alloc);
                     let alert:*const Object = msg_send!(alert, init);
                     let alert_title = NSString::alloc(nil).init_str(&"Hello World".to_string()).autorelease();
                     let alert_body = NSString::alloc(nil).init_str(&"You Clicked Me!").autorelease();

                     let _alert_id: id = msg_send!(alert, setMessageText:alert_title);
                     let _alert_id: id = msg_send!(alert, setInformativeText:alert_body);
                     let _alert_id: id = msg_send!(alert, runModal);
*/

                }
            }
            let clicked: extern fn(&Object, Sel) = clicked;
            decl.add_method(sel!(clicked), clicked);
            decl.register();
        }
    }

    fn create_button(frame: NSRect, title:String) -> *mut Object {
        unsafe {

            let button: *const Object = msg_send![class!(HelloWorldButton), alloc];
            let button_with_frame: *mut Object = msg_send![button, initWithFrame:frame];
            let title_as_nsstring = NSString::alloc(nil).init_str(&title.to_string()).autorelease();
            let _title_return: id  = msg_send![button_with_frame, setTitle:title_as_nsstring];
            let _hello_world_button_msg: id = msg_send![button_with_frame, setTarget:button_with_frame];
            let _hello_world_button_msg: id = msg_send![button_with_frame, setAction:sel!(clicked)];
            button_with_frame
        }

    }

    unsafe {

        register_button();

        let hello_world_button_frame:NSRect = NSRect::new(NSPoint::new( 0., 30.), NSSize::new(20., 10.));
        let hello_world_button = create_button(hello_world_button_frame, "Click Me".to_string());


        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // create Menu Bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        // create Application menu
        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title = quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        // create Window
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0., 0.),
                                                                      NSSize::new(200., 200.)),
                                                          NSWindowStyleMask::NSTitledWindowMask,
                                                          NSBackingStoreBuffered,
                                                          NO)
            .autorelease();
        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.center();
        let title = NSString::alloc(nil).init_str("Hello World!");
        window.setTitle_(title);
        window.makeKeyAndOrderFront_(nil);
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        window.setContentView_(hello_world_button);


        app.run();
    }
}



