use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Paths
    let output_dir = PathBuf::from("../c_output")
        .canonicalize()
        .expect("Cannot canonicalize output directory path");
    let lib_dir = PathBuf::from("../opengl_wrapper_lib")
        .canonicalize()
        .expect("Cannot canonicalize library directory path");
    let headers_path = lib_dir.join("opengl_wrapper_lib.h");
    let source_path = lib_dir.join("opengl_wrapper_lib.c");
    let obj_path = output_dir.join("opengl_wrapper_lib.o");
    let so_path = output_dir.join("libopenglwrapper.so");

    Command::new("gcc")
        .args(["-c", "-fPIC"])
        .arg(source_path.to_str().unwrap())
        .arg("-o")
        .arg(obj_path.to_str().unwrap())
        .status()
        .expect("Failed to compile the C source file");

    Command::new("gcc")
        .args(["-shared", "-o"])
        .arg(so_path.to_str().unwrap())
        .arg(obj_path.to_str().unwrap())
        .args(["-lglfw", "-lGL"])
        .status()
        .expect("Failed to create the shared library");

    // Tell cargo to look for shared libraries in the output directory
    println!("cargo:rustc-link-search={}", output_dir.to_str().unwrap());

    // Tell cargo to link the shared library
    println!("cargo:rustc-link-lib=openglwrapper");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", output_dir.to_str().unwrap());

    // Generate bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header(headers_path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
