fn main() {
    tonic_build::configure()
        .out_dir("src/generated") // Set the output directory
        .protoc_arg("-I=../proto")
        .protoc_arg("--experimental_allow_proto3_optional") // Add this line
        .compile(
            &["gdelt_service.proto",
                "some.proto",
                "event.proto",
                "mention.proto",
                "gkg.proto"], // List of proto files
            &["proto"], // Include directory for proto files
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: \n\t{:?}", e));
}