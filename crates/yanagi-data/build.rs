use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=fbs");

    let _ = flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("fbs/tables.fbs")],
        out_dir: Path::new("gen_flatbuffers"),
        ..Default::default()
    });
}
