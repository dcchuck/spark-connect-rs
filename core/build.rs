use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = fs::read_dir("./spark/connector/connect/common/src/main/protobuf/spark/connect")?;

    let mut file_paths: Vec<String> = vec![];

    for file in files {
        let entry = file?.path();
        file_paths.push(entry.to_str().unwrap().to_string());
    }

    // `read_dir` order is filesystem-dependent (ext4 hashes directory entries
    // with a per-filesystem seed), and protoc emits the generated code in input
    // order, so sort for byte-identical output across machines.
    file_paths.sort();

    #[cfg(feature = "wasm")]
    let transport = false;
    #[cfg(not(feature = "wasm"))]
    let transport = true;

    tonic_prost_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(false)
        .build_client(true)
        .build_transport(transport)
        .compile_protos(
            &file_paths,
            &["./spark/connector/connect/common/src/main/protobuf".to_string()],
        )?;

    Ok(())
}
