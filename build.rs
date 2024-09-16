use std::process::{Command, Child, Output, Stdio};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender};
use std::io::{BufRead, BufReader, Read};

fn  main()
{
    Command::new("bash")
        .arg("get_inetutils.sh")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Error launching get_inetutils.sh");
}
