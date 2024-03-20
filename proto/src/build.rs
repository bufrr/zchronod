use std::io::Result;

// used to build proto
pub fn set() -> Result<()> {
    tonic_build::configure()
        .out_dir("proto/src")
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .compile(&["src/zchronod.proto", "src/zmessage.proto", "src/msg.proto"], &["proto/"])?;
    Ok(())
}
