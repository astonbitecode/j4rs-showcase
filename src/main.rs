use std::convert::{TryFrom, TryInto};
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

use j4rs::{InvocationArg, Jvm, JvmBuilder};
use j4rs::errors::Result;
use j4rs::jfx::{FxEventType, JavaFxSupport};

fn main() -> Result<()> {
    println!("Welcome! Please select an option and hit enter:");
    println!("f: Launch a JavaFX UI built with FXML");
    println!("n: Launch a JavaFX UI built with code");
    loop {
        let input = get_string_from_stdin();
        if input == "f" {
            fxml()?;
            break;
        } else if input == "n" {
            normal()?;
            break;
        }
        println!("Please type f or n");
    }
    Ok(())
}

fn normal() -> Result<()> {
    // Create a Jvm with JavaFX support
    let jvm = JvmBuilder::new().with_javafx_support().build()?;

    // Start the JavaFX application.
    // When the JavaFX application starts, the `InstanceReceiver` channel that is returned from the `start_javafx_app` invocation
    // will receive an Instance of `javafx.stage.Stage`.
    // The UI may start being built using the provided `Stage`.
    let stage = jvm.start_javafx_app()?.rx().recv()?;

    // Create a StackPane. Java code: StackPane root = new StackPane();
    let root = jvm.create_instance("javafx.scene.layout.StackPane", &[])?;

    // Create the button. Java code: Button btn = new Button();
    let btn = jvm.create_instance("javafx.scene.control.Button", &[])?;
    // Get the action channel for this button
    let btn_action_channel = jvm.set_javafx_event_receiver(&btn, "setOnAction")?;
    // Set the text of the button. Java code: btn.setText("Say Hello World to Rust");
    jvm.invoke(&btn, "setText", &["A button that sends events to Rust".try_into()?])?;
    // Add the button to the GUI. Java code: root.getChildren().add(btn);
    jvm.chain(&root)?
        .invoke("getChildren", &[])?
        .invoke("add", &[btn.try_into()?])?
        .collect();

    // Create a new Scene. Java code: Scene scene = new Scene(root, 300, 250);
    let scene = jvm.create_instance("javafx.scene.Scene", &[
        root.try_into()?,
        InvocationArg::try_from(300_f64)?.into_primitive()?,
        InvocationArg::try_from(250_f64)?.into_primitive()?])?;
    // Set the title for the scene. Java code: stage.setTitle("Hello Rust world!");
    jvm.invoke(&stage, "setTitle", &["Hello Rust world!".try_into()?])?;
    // Set the scene in the stage. Java code: stage.setScene(scene);
    jvm.invoke(&stage, "setScene", &[scene.try_into()?])?;
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

fn fxml() -> Result<()> {
    // Create a Jvm with JavaFX support
    let jvm = JvmBuilder::new().with_javafx_support().build()?;

    // Start the JavaFX application.
    // When the JavaFX application starts, the `InstanceReceiver` channel that is returned from the `start_javafx_app` invocation
    // will receive an Instance of `javafx.stage.Stage`.
    // The UI may start being built using the provided `Stage`.
    let stage = jvm.start_javafx_app()?.rx().recv()?;

    // Set the title for the scene. Java code: stage.setTitle("Hello Rust world!");
    jvm.invoke(&stage, "setTitle", &["Hello JavaFX from Rust!".try_into()?])?;
    // Show the stage. Java code: stage.show();
    jvm.invoke(&stage, "show", &[])?;

    // Get the onclose handler channel
    let onclose_channel = jvm.on_close_event_receiver(&stage)?;

    // Load a fxml
    let controller = jvm.load_fxml(&PathBuf::from("./fxml/jfx_in_rust.fxml"), &stage)?;

    // Wait for the controller to be initialized. This is not mandatory, it is here to shoe that the functionality exists.
    let _ = controller.on_initialized_callback(&jvm)?.rx().recv()?;
    println!("The controller is initialized!");

    // Get the InstanceReceiver to retrieve callbacks from the JavaFX button with id helloButton
    let hello_button_action_channel = controller.get_event_receiver_for_node("helloButton", FxEventType::ActionEvent_Action, &jvm)?;

    loop {
        let (index, _event) = Jvm::select(&[&onclose_channel, &hello_button_action_channel])?;
        if index == 0 {
            println!("Exiting");
            break;
        } else {
            println!("Hello Button clicked!");
        }
    }

    Ok(())
}

fn get_string_from_stdin() -> String {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}
