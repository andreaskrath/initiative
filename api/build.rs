use std::process::Command;

const NPM: &str = "npm";
const VIEW_PATH: &str = "../view";

fn main() {
    println!("cargo:rerun-if-env-changed=BUILD_FRONTEND");
    println!("cargo:rerun-if-changed=../view/src");

    if std::env::var("BUILD_FRONTEND").unwrap_or_default() != "true" {
        return;
    }

    let npm_install = Command::new(NPM)
        .current_dir(VIEW_PATH)
        .arg("install")
        .status()
        .expect("failed to install node modules");

    if !npm_install.success() {
        panic!("failed to install node modules");
    }

    let npm_build = Command::new(NPM)
        .current_dir(VIEW_PATH)
        .args(["run", "build"])
        .status()
        .expect("failed to build frontend");

    if !npm_build.success() {
        panic!("failed to build frontend")
    }
}
