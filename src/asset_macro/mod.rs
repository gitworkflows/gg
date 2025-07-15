//! Defines a macro for embedding assets into the binary at compile time.

/// Embeds a file into the binary as a `&'static [u8]`.
///
/// This macro is a wrapper around `include_bytes!`, providing a consistent
/// way to include assets.
///
/// # Examples
///
/// \`\`\`ignore
/// // In your Cargo.toml:
/// // [build-dependencies]
/// // ... (no special build script needed for include_bytes!)
///
/// // In your Rust code:
/// let my_asset = include_asset!("path/to/your/asset.txt");
/// println!("Asset content: {:?}", my_asset);
/// \`\`\`
#[macro_export]
macro_rules! include_asset {
    ($path:expr) => {{
        // This macro will embed the content of the file at $path into the binary.
        // The path is relative to the Cargo.toml of the crate where the macro is used.
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
    }};
}

pub fn get_asset_path(_name: &str) -> String {
    // Dummy function - actual assets are embedded via macro
    format!("/path/to/assets/{}", _name)
}
