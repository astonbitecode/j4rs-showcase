use std::convert::TryFrom;

use j4rs::{InvocationArg, JvmBuilder, Jvm};
use j4rs::errors::Result;
use j4rs::jfx::JavaFxSupport;

fn main() -> Result<()> {
    // Create a Jvm with JavaFX support
    let jvm = JvmBuilder::new().with_javafx_support().build().unwrap();

    // Start the JavaFX application.
    // When the JavaFX application starts, the `InstanceReceiver` channel that is returned from the `start_javafx_app` invocation
    // will receive an Instance of `javafx.stage.Stage`.
    // The UI may start being built using the provided `Stage`.
    let stage = jvm.start_javafx_app()?.rx().recv().unwrap();

    // Create a StackPane. Java code: StackPane root = new StackPane();
    let root = jvm.create_instance("javafx.scene.layout.StackPane", &[])?;

    // Create the button. Java code: Button btn = new Button();
    let btn = jvm.create_instance("javafx.scene.control.Button", &[])?;
    // Get the action channel for this button
    let btn_action_channel = jvm.set_javafx_event_receiver(&btn, "setOnAction")?;
    // Set the text of the button. Java code: btn.setText("Say Hello World to Rust");
    jvm.invoke(&btn, "setText", &[InvocationArg::try_from("Say Hello World to Rust")?])?;
    // Add the button to the GUI. Java code: root.getChildren().add(btn);
    jvm.chain(&root)?
        .invoke("getChildren", &[])?
        .invoke("add", &[InvocationArg::try_from(btn)?])?
        .collect();

    // Create a new Scene. Java code: Scene scene = new Scene(root, 300, 250);
    let scene = jvm.create_instance("javafx.scene.Scene", &[
        InvocationArg::try_from(root)?,
        InvocationArg::try_from(300_f64)?.into_primitive()?,
        InvocationArg::try_from(250_f64)?.into_primitive()?])?;
    // Set the title for the scene. Java code: stage.setTitle("Hello Rust world!");
    jvm.invoke(&stage, "setTitle", &[InvocationArg::try_from("Hello Rust world!")?])?;
    // Set the scene in the stage. Java code: stage.setScene(scene);
    jvm.invoke(&stage, "setScene", &[InvocationArg::try_from(scene)?])?;
    // Show the stage. Java code: stage.show();
    jvm.invoke(&stage, "show", &[])?;

    // Get the onclose handler channel
    let onclose_channel = jvm.on_close_event_receiver(&stage)?;

    loop {
        let (index, _event) = Jvm::select(&[&btn_action_channel, &onclose_channel])?;
        if index == 1 {
            println!("Exiting");
            break;
        } else {
            println!("Button clicked");
        }
    }

    Ok(())
}
