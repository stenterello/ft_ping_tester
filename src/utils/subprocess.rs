use std::cell::RefCell;
use std::io::{BufRead, BufReader};
use std::panic;
use std::process::{self, ChildStderr, ChildStdout, Command, ExitStatus, Stdio};
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct SubProcess {
    path: String,
    tx: Sender<String>,
    pub exit_status: RefCell<ExitStatus>,
}

impl SubProcess {
    pub fn new(path: String, tx: Sender<String>) -> Self {
        SubProcess {
            path,
            tx,
            exit_status: RefCell::new(ExitStatus::default()),
        }
    }

    pub fn start(&self, args: Vec<String>) -> () {
        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            orig_hook(panic_info);
            process::exit(1);
        }));

        let mut child = Command::new(self.path.clone())
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout: ChildStdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        let stderr: ChildStderr = child.stderr.take().unwrap();
        let error_reader = BufReader::new(stderr);

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            self.tx.send(line).expect("Failed to send line");
        }
        for line in error_reader.lines() {
            let line = line.expect("Failed to read line");
            self.tx.send(line).expect("Failed to send line");
        }

        let mut stat = self.exit_status.borrow_mut();
        match child.try_wait() {
            Ok(Some(status)) => {
                *stat = status;
            }
            _ => {}
        }
    }
}
