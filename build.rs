use j4rs::JvmBuilder;
use j4rs::jfx::JavaFxSupport;

fn main() {
    let jvm = JvmBuilder::new().build().unwrap();
    jvm.deploy_javafx_dependencies().unwrap();
}
