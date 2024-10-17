use std::{env, path::PathBuf};

// sync tigerbeetle version with crate vers
pub const TB_VERS: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let profile = std::env::var("PROFILE").unwrap();

    let tigerbeetle_root_path = PathBuf::from("tigerbeetle")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = tigerbeetle_root_path.join("src/clients/c/tb_client.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    let libdir_path = tigerbeetle_root_path.join("zig-out/lib");
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=tb_client");

    // Run zig build here to build libtb_client.a
    let should_build_release_mode = profile.as_str() == "release";
    if !std::process::Command::new("zig")
        .arg("build")
        .arg(format!("-Dconfig-release={TB_VERS}"))
        // important: clients with lower versions will not be allowed to interact with cluster
        // and will have their features restricted to the version they're on
        .arg(format!("-Dconfig-release-client-min={TB_VERS}"))
        .arg(format!("-Drelease={should_build_release_mode}"))
        .current_dir(tigerbeetle_root_path)
        .output()
        .expect("could not spawn `zig`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not zig build static library");
    }

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
