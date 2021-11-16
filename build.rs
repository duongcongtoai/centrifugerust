fn main() {
    tonic_build::configure()
        .compile(&["proto/api.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
