
extern crate cc;

fn main() {

    println!(r"cargo:rustc-link-search=/Users/anselm/orbital/makepad/pluto/avtest");

    println!("cargo:rustc-link-lib=framework=AVFoundation");

    println!("cargo:rerun-if-changed=avtest/avcapture.h");

	// I could try this for avfoundation
	// https://simlay.net/posts/rust-bindgen-objc-support/

// https://github.com/simlay/uikit-sys/blob/master/build.rs
//    let mut clang_args = vec!["-x", "objective-c", "-fblocks", &target_arg];
//    if let Some(sdk_path) = sdk_path {
//        clang_args.extend(&["-isysroot", sdk_path]);
//    }

//let sdk_path = "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS13.6.sdk";

//let sdk_path = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/AVFoundation.framework";

let sdk_path = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";

// /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/AVFoundation.framework/Versions/A/Headers/AVFoundation.h

    let builder = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .header("avtest/avtest.h")
//        .clang_args(&[&format!("--target={}", target)])
        .clang_args(&["-isysroot", sdk_path])
        .block_extern_crate(true)
        .generate_block(true)
        .clang_args(&["-fblocks"])
        .objc_extern_crate(true)
        .clang_args(&["-x", "objective-c"])
        .blacklist_item("timezone")
        .blacklist_item("IUIStepper")
        .blacklist_function("dividerImageForLeftSegmentState_rightSegmentState_")
        .blacklist_item("objc_object");


    let bindings = builder.generate().expect("unable to generate bindings");

    bindings
        .write_to_file("avtest/avtestbind.in")
        .expect("could not write bindings");

/*
    // For C++ This is generating bindings that do not work...so the source is declaring them again as well
    let bindings = bindgen::Builder::default()
        .header("avtest/avtest.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    //let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    //bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");

    bindings.write_to_file("avtest/avtest_bindings.rs").expect("Couldn't write bindings!");
*/


/* This works for C++ but not for objective c - so i build by hand for now

    cc::Build::new()
		.file("avtest/avtest.m")
		.compile("libavcapture.a");
*/

}
