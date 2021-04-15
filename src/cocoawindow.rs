

// This has to be first
#![allow(non_snake_case)]

// Macros annoyingly have to be specified above in main.rs
//#[macro_use] extern crate objc;

// Not used anymore...
// use std::ffi::CString;

// I'm having trouble with dispatching - this is a dispatcher
//extern crate dispatch;
//use dispatch::{Queue,QueueAttribute};

// Here are some helpful tools
//extern crate cocoa;
use cocoa::base::{selector, nil, id, NO, YES};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSBackingStoreBuffered, NSMenu, NSMenuItem, NSWindowStyleMask,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps};

// More helpful tools
use objc::runtime::{Class, Object, Sel, Protocol};
use objc::declare::ClassDecl;

// NSLog was not given to us by anybody so let's pull it in ourselves directly from Foundation
//#[link(name = "Foundation", kind = "framework")] <- alread provided
extern { pub fn NSLog(fmt: id, ...); }

/* some code to open a display


fn opendisplay() {

    fn register_button() {
        unsafe {
            let superclass = class!(NSButton);
            let mut decl = ClassDecl::new("HelloWorldButton", superclass).unwrap();

            extern fn clicked(_this: &Object, _cmd: Sel) {
//                unsafe {
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

  //              }
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


*/






