
extern crate cc;

fn main() {

    // AVFoundation is needed - can be done here or could be pulled in right from rust source
    println!("cargo:rustc-link-lib=framework=AVFoundation");

    // force link in libavtest.a by providing library path
    println!(r"cargo:rustc-link-search=/Users/anselm/orbital/makepad/pluto/avtest");

    // ...
    println!("cargo:rerun-if-changed=avtest/avtest.h");

/*
    // Use this approach to generate avfoundation headers
    // turned off for now....

	// https://simlay.net/posts/rust-bindgen-objc-support/

    let sdk_path = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";

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
*/

/*
    // Generate simpler bindings for a bare bones headers without pulling in all of avfoundation
    // Also turned off for now ... I just build these by hand

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


/*

    // This works for C++ but not for objective c - so i build by hand for now also...
    // goto the avtest folder and run ./make by hand to build libavtest.a

    cc::Build::new()
		.file("avtest/avtest.m")
		.compile("libavtest.a");
*/

}
