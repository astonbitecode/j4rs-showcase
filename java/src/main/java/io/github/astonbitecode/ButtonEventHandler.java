package io.github.astonbitecode;

import javafx.event.ActionEvent;
import javafx.event.EventHandler;
import org.astonbitecode.j4rs.api.invocation.NativeCallbackToRustChannelSupport;

/**
 * An {@link EventHandler} that extends {@link NativeCallbackToRustChannelSupport}.
 *
 * This simple handler is used to send callbacks to Rust when an {@link ActionEvent} is received.
 */
public class ButtonEventHandler extends NativeCallbackToRustChannelSupport implements EventHandler<ActionEvent> {
    private int i = 0;

    @Override
    public void handle(ActionEvent event) {
        // Send a String to the Rust world...
        doCallback(++i + ": Hello from the Java world!");
    }
}
