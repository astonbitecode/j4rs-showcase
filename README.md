# j4rs-showcase

Showcase for a Rust application that creates and uses a JavaFX User Interface.

## Steps to execute the showcase

1. Build with

    ```bash
    cargo build --manifest-path=rust/Cargo.toml
    ``` 
    
    During building, you should get a warning like:

    >  warning: Please export the env var "FX_UI_JAR_PATH" with the value "/home/myuser/git/j4rs-showcase/java/target/fx-ui-0.1.0.jar" before running the application

1. Export the FX_UI_JAR_PATH variable
    
    For linux:
    ```bash
    export FX_UI_JAR_PATH=/home/myuser/git/j4rs-showcase/java/target/fx-ui-0.1.0.jar
    ```

1. Execute with

    ```bash
    cargo run --manifest-path=rust/Cargo.toml
    ```