extern crate libc;

use std::cell::RefCell;
use std::io::{BufRead, BufReader, Result};
use std::process::{Child, ChildStderr, ChildStdout, Command, ExitStatus, Stdio};
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

    pub fn start(&self, args: Vec<String>) -> Result<()> {
        let child = Command::new(self.path.clone())
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let unwrapped_child: Option<Child>;

        match child {
            Ok(inner_child) => {
                unwrapped_child = Some(inner_child);
            }
            Err(e) => return Err(e),
        };

        let mut child = unwrapped_child.unwrap();

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
        };
        Ok(())
    }
}
