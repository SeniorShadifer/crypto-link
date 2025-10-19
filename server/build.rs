fn main() {
    let cargo_root = std::path::Path::new(&env!("CARGO_MANIFEST_DIR"));
    let workspace_root = cargo_root.join("..");
    let profile = std::env::var("CARGO_PROFILE").unwrap_or_else(|_| "debug".to_string());
    std::fs
        ::copy(
            cargo_root.join("default.conf"),
            workspace_root.join("target").join(profile).join("default.conf")
        )
        .unwrap();
}
