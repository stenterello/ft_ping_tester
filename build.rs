use std::process::{Command, Child, Output, Stdio};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender};
use std::io::{BufRead, BufReader, Read};

fn  main()
{
    println!("cargo:warning=Launching get_inetutils.sh");
    
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (error_tx, error_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let handle = thread::spawn(|| {
        let mut child = Command::new("bash")
            .arg("get_inetutils.sh")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Error launching get_inetutils.sh");

        let output_reader = BufReader::new(child.stdout.take().unwrap());
        let handle2 = thread::spawn(move || {
            for line in output_reader.lines() {
                let line = line.expect("Failed to read line");
                tx.send(line).expect("Failed to send line");
            }
        });

        handle2.join();
    });

    /*if !output.stdout.is_empty() {
        println!("cargo:warning={}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        println!("cargo:warning={}", String::from_utf8_lossy(&output.stderr));
    }*/

    handle.join();
}
