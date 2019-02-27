use std::process::Command;
use std::path::PathBuf;
use std::fs::{canonicalize, File};

fn main() {
    build_java();

    let java_jar_rel_path = PathBuf::from("../java/target/fx-ui-0.1.0.jar");
    let _ = File::open(&java_jar_rel_path).expect("Error while accessing the java jar file");
    let canonicalized = canonicalize(&java_jar_rel_path).expect("Cannot get the absolute path of the jar file");
    let java_jar_abs_path = canonicalized.to_str().unwrap();
    println!("cargo:warning=Please export the env var \"FX_UI_JAR_PATH\" with the value \"{}\" before running the application", java_jar_abs_path);
}

fn build_java() {
    let mut command = Command::new("mvn");

    command.arg("clean");
    command.arg("install");
    command.arg("-f");
    command.arg("../java/pom.xml");

    let output = command.output().expect("Could not build Java");
    if !output.status.success() {
        panic!("{}", String::from_utf8(output.stdout).expect("Could not get the std err to display the error..."));
    }
}