extern crate libc;

use std::cell::RefCell;
use std::io::{BufRead, BufReader, Result};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;

#[derive(Debug)]
pub struct SubProcess {
    path: String,
    tx: Sender<String>,
    pub exit_code: RefCell<i32>,
}

impl SubProcess {
    pub fn new(path: String, tx: Sender<String>) -> Self {
        SubProcess {
            path,
            tx,
            exit_code: RefCell::new(1),
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
        let reader = BufReader::new(child.stdout.take().unwrap());
        let error_reader = BufReader::new(child.stderr.take().unwrap());

        let tx = self.tx.clone();
        thread::spawn(move || {
            for line in reader.lines() {
                let line = line.expect("Failed to read line");
                tx.send(line).expect("Failed to send line");
            }
        });

        let tx = self.tx.clone();
        thread::spawn(move || {
            for line in error_reader.lines() {
                let line = line.expect("Failed to read line");
                tx.send(line).expect("Failed to send line");
            }
        });

        let mut stat = self.exit_code.borrow_mut();
        match child.wait() {
            Ok(r) => *stat = r.code().unwrap(),
            _ => {}
        };
        Ok(())
    }
}
