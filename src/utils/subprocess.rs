extern crate libc;

use std::cell::RefCell;
use std::io::{BufRead, BufReader, Read, Result};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;

#[derive(Debug)]
pub struct SubProcess {
    path: String,
    tx: Sender<String>,
    error_tx: Sender<String>,
    pub exit_code: RefCell<Option<i32>>,
}

impl SubProcess {
    pub fn new(path: String, tx: Sender<String>, error_tx: Sender<String>) -> Self {
        SubProcess {
            path,
            tx,
            error_tx,
            exit_code: RefCell::new(None),
        }
    }

    fn send_lines<R: Read + Send + 'static>(&self, reader: BufReader<R>, tx: Sender<String>) {
        thread::spawn(move || {
            for line in reader.lines() {
                let line = line.expect("Failed to read line");
                tx.send(line).expect("Failed to send line");
            }
        });
    }

    pub fn start(&mut self, args: Vec<String>) -> Result<()> {
        let mut stat = self.exit_code.borrow_mut();
        let mut child: Child = match Command::new(self.path.clone())
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(a) => a,
            Err(e) => {
                *stat = Some(1);
                return Err(e);
            }
        };

        self.send_lines(
            BufReader::new(child.stdout.take().unwrap()),
            self.tx.clone(),
        );
        self.send_lines(
            BufReader::new(child.stderr.take().unwrap()),
            self.error_tx.clone(),
        );

        match child.wait() {
            Ok(r) => {
                *stat = Some(r.code().unwrap());
            }
            _ => {
                *stat = Some(1);
            }
        };
        Ok(())
    }
}
