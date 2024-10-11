fn main() {
    tonic_build::configure()
        .build_server(true)  // Generate server-side code
        .compile(&["proto/search_service.proto"], &["proto"])  // Path to your .proto files
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
