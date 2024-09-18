use std::cell::RefCell;
use std::io::{BufRead, BufReader, Read, Result};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;

#[derive(Debug)]
pub struct SubProcess {
    path: String,
    name: String,
    tx: Sender<String>,
    error_tx: Sender<String>,
    pub exit: RefCell<(Option<i32>, Option<String>)>,
}

impl SubProcess {
    pub fn new(path: String, name: String, tx: Sender<String>, error_tx: Sender<String>) -> Self {
        SubProcess {
            path,
            name,
            tx,
            error_tx,
            exit: RefCell::new((None, None)),
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
        let mut stat = self.exit.borrow_mut();
        let mut child: Child = match Command::new(self.path.clone() + self.name.clone().as_str())
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(a) => a,
            Err(e) => {
                *stat = (None, Some(e.to_string()));
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
                *stat = (
                    Some(match r.code() {
                        Some(n) => n,
                        None => 127,
                    }),
                    None,
                );
            }
            Err(e) => {
                *stat = (None, Some(e.to_string()));
            }
        };
        Ok(())
    }
}
