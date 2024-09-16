use std::process::Command;

fn main() {
    Command::new("bash")
        .arg("get_inetutils.sh")
        .output()
        .expect("Error launching get_inetutils.sh");
}
