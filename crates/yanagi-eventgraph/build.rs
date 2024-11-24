use std::{fs, path::Path};

pub fn main() {
    let proto_file = "action_info.proto";
    if Path::new(&proto_file).exists() {
        println!("cargo:rerun-if-changed={proto_file}");
        let _ = fs::create_dir("out/");

        prost_build::Config::new()
            .out_dir("out/")
            .compile_protos(&[proto_file], &["."])
            .unwrap();
    }
}
