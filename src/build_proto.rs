use std::io::Result;
fn main() -> Result<()> {
    println!("cargo:warning=STARTING");
    prost_build::compile_protos(&["proto/SignalService.proto", "proto/WebSocketResources.proto"], &["proto/"])?;
    println!("cargo:warning=DONE: `fd signal target` to find .rs");
    Ok(())
}
