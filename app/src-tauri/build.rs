const PROTO_FILE_PATH: &str = "interface/interface.proto";
const OUT_DIR: &str = "src/generated";

fn main() {
    println!("cargo:rerun-if-changed={}", PROTO_FILE_PATH);
    // Compile the proto files and place the generated code in the `OUT_DIR`
    prost_build::Config::new()
        .out_dir(OUT_DIR) // Specify the output directory
        .compile_protos(&[PROTO_FILE_PATH], &["proto"])
        .expect("Failed to compile protos");

    tauri_build::build()
}
