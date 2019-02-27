package io.github.astonbitecode;

import javafx.application.Application;
import javafx.scene.Scene;
import javafx.scene.control.Button;
import javafx.scene.layout.StackPane;
import javafx.stage.Stage;

public class FxUiApp extends Application {
    private static StackPane root = new StackPane();

    @Override
    public void start(Stage primaryStage) {
        // Create the button
        Button btn = new Button();
        btn.setText("Say Hello World to Rust");
        btn.setOnAction(new ButtonEventHandler());
        root.getChildren().add(btn);

        Scene scene = new Scene(root, 300, 250);

        primaryStage.setTitle("Hello Rust world!");
        primaryStage.setScene(scene);
        primaryStage.show();
    }

    public static void launchFxUiApp() {
        new Thread(() -> {
            // Launch the javafx app
            Application.launch();
        }).start();
    }

    public static ButtonEventHandler getButtonEventHandler() throws Exception {
        while (root.getChildren().isEmpty())
            Thread.sleep(100);
        return (ButtonEventHandler)((Button)root.getChildren().get(0)).getOnAction();
    }
}
