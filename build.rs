
// extern crate cc;

fn main() {

    // AVFoundation is needed - can be done here or could be pulled in right from rust source
    // println!("cargo:rustc-link-lib=framework=AVFoundation");

    /*

    // No longer pulled in
    // This was a test of calling back to a linked objective-c module to do some of the work
    // See https://simlay.net/posts/rust-bindgen-objc-support/

    // println!(r"cargo:rustc-link-search=/Users/anselm/orbital/makepad/pluto/avtest");
    // println!("cargo:rerun-if-changed=avtest/avtest.h");

    let sdk_path = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";

    let builder = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .header("avtest/avtest.h")
        //.clang_args(&[&format!("--target={}", target)])
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
    bindings.write_to_file("avtest/avtestbind.in").expect("could not write bindings");

    */

    /*

    // Unused code - generate bindings for some c / c++ code 
    // NOTE C bindings work but C++ bindings are not linking; some kind of name mangling obviously
    // I ended up just writing a make script in the avtest folder and building by hand

    let bindings = bindgen::Builder::default()
        .header("avtest/avtest.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    //let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    //bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
    bindings.write_to_file("avtest/avtest_bindings.rs").expect("Couldn't write bindings!");

    cc::Build::new()
		.file("avtest/avtest.m")
		.compile("libavtest.a");
    */

}
