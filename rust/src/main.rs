use j4rs::ClasspathEntry;
use std::env;

fn main() {
    // Get the classpath of the
    let fx_jar_path = env::var("FX_UI_JAR_PATH").expect("The FX_UI_JAR_PATH env var is not set.");

    println!("{}", fx_jar_path);

    let cp_entry = ClasspathEntry::new(&fx_jar_path);

    let jvm_res = j4rs::JvmBuilder::new()
        .classpath_entry(cp_entry)
        .build();

    let jvm = jvm_res.unwrap();

    let _ = jvm.invoke_static(
        "io.github.astonbitecode.FxUiApp",
        "launchFxUiApp",
        &vec![])
        .unwrap();

    let button_event_handler = jvm.invoke_static(
        "io.github.astonbitecode.FxUiApp",
        "getButtonEventHandler",
        &vec![])
        .unwrap();

    let instance_receiver = jvm.init_callback_channel(&button_event_handler).unwrap();

    while let Ok(received) = instance_receiver.rx().recv() {
        let message_from_java: String = jvm.to_rust(received).unwrap();
        println!("Rust received: {}", message_from_java);
    }
}
