use std::env;
use j4rs::{ClasspathEntry, InstanceReceiver};

fn main() {
    // We need to add the jar file that contains
    // the UI code in the classpath.
    // We assume that the env var "FX_UI_JAR_PATH" exists
    // and contains the absolute path of the jar file.
    // E.g. /home/myuser/git/j4rs-showcase/java/target/fx-ui-0.1.0.jar
    let ui_jar_path = env::var("FX_UI_JAR_PATH").expect("The FX_UI_JAR_PATH env var is not set.");

    // Create the `ClasspathEntry` for the jar
    let cp_entry = ClasspathEntry::new(&ui_jar_path);

    // Create a new Jvm, adding the FX_UI_JAR_PATH to the classpath
    let jvm = j4rs::JvmBuilder::new()
        .classpath_entry(cp_entry)
        .build().expect("Could not create the JVM");

    // Launch the JavaFX UI
    let _ = jvm.invoke_static(
        "io.github.astonbitecode.FxUiApp",
        "launchFxUiApp",
        &vec![])
        .unwrap();

    // Retrieve the ButtonEventHandler instance
    // that is used to handle the JavaFX button events.
    let button_event_handler = jvm.invoke_static(
        "io.github.astonbitecode.FxUiApp",
        "getButtonEventHandler",
        &vec![])
        .unwrap();

    // Install a callback channel in the ButtonEventHandler instance
    let instance_receiver: InstanceReceiver = jvm
        .init_callback_channel(&button_event_handler).unwrap();

    // Wait for messages from the receiver
    while let Ok(received) = instance_receiver.rx().recv() {
        let message_from_java: String = jvm.to_rust(received).unwrap();
        println!("Rust received: {}", message_from_java);
    }
}
