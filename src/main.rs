use std::env;
use std::fs;
use std::process::Command;

mod crate_toml;
mod workspace_toml;
use crate_toml::create_crate_toml;
use workspace_toml::create_workspace_toml;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len < 3 {
        eprint!("->> usage: mephi <bin/lib/ws> <project_name>");
        std::process::exit(1);
    }

    let project_type = &args[1];
    let project_name = &args[2];

    match project_type.as_str() {
        "bin" | "lib" => create_crate(project_type, project_name),
        "ws" => create_workspace(project_name),
        _ => {
            eprint!("->> usage: mephi <bin/lib/ws> <project_name>");
            std::process::exit(1);
        }
    }
}

fn create_crate(project_type: &str, project_name: &str) {
    let project_type = &format!("--{}", project_type);

    let cmd = Command::new("cargo")
        .arg("new")
        .arg(project_type)
        .arg(project_name)
        .status()
        .expect(&format!(
            "->> failed to execute `cargo new {} {}`",
            project_type, project_name
        ));

    if !cmd.success() {
        eprint!("->> `cargo new {} {}` failed", project_type, project_name);
        std::process::exit(1);
    }

    print!("->> vroom vroom, `{}`", project_name);

    let crate_toml_string = create_crate_toml(project_name);
    let cargo_toml_path = format!("./{}/Cargo.toml", project_name);

    fs::write(cargo_toml_path, crate_toml_string).expect("->> failed to overwrite `Cargo.toml`");
}

fn create_workspace(project_name: &str) {
    let cargo_toml_string = create_workspace_toml();

    fs::create_dir_all(project_name).expect("->> failed to create the workspace directory");
    fs::write(format!("./{}/Cargo.toml", project_name), cargo_toml_string)
        .expect("->> failed to create `Cargo.toml`");

    print!("->> vroom vroom, `{}`", project_name);
}
