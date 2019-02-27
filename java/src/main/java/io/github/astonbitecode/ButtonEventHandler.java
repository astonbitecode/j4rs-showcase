package io.github.astonbitecode;

import javafx.event.ActionEvent;
import javafx.event.EventHandler;
import org.astonbitecode.j4rs.api.invocation.NativeCallbackToRustChannelSupport;

public class ButtonEventHandler extends NativeCallbackToRustChannelSupport implements EventHandler<ActionEvent> {
    private int i = 0;

    @Override
    public void handle(ActionEvent event) {
        doCallback(++i + ": Hello from the Java world!");
    }
}
