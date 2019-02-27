package io.github.astonbitecode;

import javafx.application.Application;
import javafx.scene.Scene;
import javafx.scene.control.Button;
import javafx.scene.layout.StackPane;
import javafx.stage.Stage;

/**
 * A simple JavaFX application that contains just a Button
 */
public class FxUiApp extends Application {
    // The application instance is created by the JavaFX framework.
    // We need a way to access the instance from Rust though,
    // in order to initialize the callbacks.
    //
    // To keep things simple for this example,
    // we keep here the root of the JavaFX Scene.
    // From this, we will be able to get the ButtonEventHandler
    // and setup the rust callbacks in order for Java
    // to communicate with Rust.
    private static StackPane root = new StackPane();

    @Override
    public void start(Stage primaryStage) {
        // Create the button
        Button btn = new Button();
        btn.setText("Say Hello World to Rust");
        btn.setOnAction(new ButtonEventHandler());
        root.getChildren().add(btn);

        // Create the Scene
        Scene scene = new Scene(root, 300, 250);

        primaryStage.setTitle("Hello Rust world!");
        primaryStage.setScene(scene);
        primaryStage.show();
    }

    // Rust code will call this method to launch the JavaFX application
    public static void launchFxUiApp() {
        new Thread(() -> {
            // Launch the javafx app
            Application.launch();
        }).start();
    }

    // Rust code will call this method to retrieve the
    // ButtonEventHandler instance that handles events of the Button.
    // Rust code will use this instance to initialize a callback channel.
    public static ButtonEventHandler getButtonEventHandler() throws Exception {
        // Wait until the JavaFX launches.
        // This is a hack to keep things readable.
        // Better approaches may include using latches, atomic references etc.
        while (root.getChildren().isEmpty())
            Thread.sleep(100);
        // We know exactly what is being created in this example.
        // This is totally unsafe for other use cases.
        return (ButtonEventHandler)((Button)root.getChildren().get(0)).getOnAction();
    }
}
