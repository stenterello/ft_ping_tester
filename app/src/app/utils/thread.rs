use crate::app::widgets::traits::viewer::OutputType;

use super::subprocess::SubProcess;
use std::cell::RefCell;
use std::io::Result;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub struct Thread {
    command: Arc<Mutex<SubProcess>>,
    rx: Receiver<String>,
    error_rx: Receiver<String>,
    output: RefCell<Vec<String>>,
    error_output: RefCell<Vec<String>>,
    pub name: String,
    handle: Option<JoinHandle<Result<()>>>,
}

impl Thread {
    pub fn new(path: &str, name: &str) -> Self {
        let (tx, rx) = mpsc::channel();
        let (error_tx, error_rx) = mpsc::channel();
        Self {
            command: Arc::new(Mutex::new(SubProcess::new(
                path.to_string(),
                name.to_string(),
                tx,
                error_tx,
            ))),
            rx,
            error_rx,
            output: RefCell::new(Vec::default()),
            error_output: RefCell::new(Vec::default()),
            name: name.to_string(),
            handle: None,
        }
    }

    pub fn start(&mut self, args: Vec<String>) {
        let command = Arc::clone(&self.command);
        let handle = move || -> Result<()> {
            let mut command = command.lock().unwrap();
            command.start(args)
        };

        let thread = thread::Builder::new().name(self.name.clone());

        self.handle = Some(thread.spawn(handle).unwrap());
    }

    pub fn get_output(&self, t: OutputType) -> Vec<String> {
        let (receiver, output): (&Receiver<String>, &RefCell<Vec<String>>) = match t {
            OutputType::Stdout => (&self.rx, &self.output),
            OutputType::Stderr => (&self.error_rx, &self.error_output),
        };
        match receiver.try_recv() {
            Ok(received) => output.borrow_mut().push(received),
            _ => {}
        };
        output.borrow().clone()
    }

    pub fn take_output(&self, t: OutputType) -> Vec<String> {
        let (receiver, output): (&Receiver<String>, &RefCell<Vec<String>>) = match t {
            OutputType::Stdout => (&self.rx, &self.output),
            OutputType::Stderr => (&self.error_rx, &self.error_output),
        };
        match receiver.try_recv() {
            Ok(received) => output.borrow_mut().push(received),
            _ => {}
        };
        output.take()
    }

    pub fn is_running(&self) -> bool {
        match &self.handle {
            Some(t) => !t.is_finished(),
            None => false,
        }
    }

    pub fn get_exit(&self) -> (Option<i32>, Option<String>) {
        if self.is_running() {
            (None, None)
        } else {
            match &*self.command.lock().unwrap().exit.borrow() {
                (Some(r), None) => (Some(*r), None),
                (Some(code), Some(err)) => (Some(*code), Some(err.to_string())),
                (None, _) => (None, None),
            }
        }
    }

    pub fn clear_buffers(&mut self) {
        self.output.borrow_mut().clear();
        self.error_output.borrow_mut().clear();
    }
}
