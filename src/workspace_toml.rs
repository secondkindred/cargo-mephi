pub fn create_workspace_toml() -> String {
    format!(
        r#"cargo-features = ["codegen-backend"]

[workspace]
resolver = "3"
members = []

[workspace.dependencies]

[profile.release]
codegen-backend = "llvm"
opt-level = 3
debug = false
split-debuginfo = "..."
strip = true
debug-assertions = false
overflow-checks = false
lto = "fat"
panic = "unwind"
incremental = false
codegen-units = 1
rpath = false

[profile.release.build-override]
codegen-backend = "llvm"
opt-level = 3
debug = false
split-debuginfo = "..."
strip = true
debug-assertions = false
overflow-checks = false
incremental = false
codegen-units = 1

[profile.dev]
codegen-backend = "cranelift"
opt-level = 0
debug = true
split-debuginfo = "..."
strip = "none"
debug-assertions = true
overflow-checks = true
lto = "off"
panic = "abort"
incremental = true
codegen-units = 384
rpath = false

[profile.dev.build-override]
codegen-backend = "cranelift"
opt-level = 0
debug = true
split-debuginfo = "..."
strip = "none"
debug-assertions = true
overflow-checks = true
incremental = true
codegen-units = 384"#
    )
}
