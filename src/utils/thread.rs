use crate::utils::subprocess::SubProcess;
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
    pub fn new(path: String, name: String) -> Self {
        let (tx, rx) = mpsc::channel();
        let (error_tx, error_rx) = mpsc::channel();
        Self {
            command: Arc::new(Mutex::new(SubProcess::new(
                path.clone(),
                name.clone(),
                tx,
                error_tx,
            ))),
            rx,
            error_rx,
            output: RefCell::new(Vec::default()),
            error_output: RefCell::new(Vec::default()),
            name,
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

    pub fn get_output(&self) -> Vec<String> {
        match self.rx.try_recv() {
            Ok(received) => self.output.borrow_mut().push(received),
            _ => {}
        };
        self.output.borrow().clone()
    }

    pub fn get_error_output(&self) -> Vec<String> {
        match self.error_rx.try_recv() {
            Ok(received) => self.error_output.borrow_mut().push(received),
            _ => {}
        };
        self.error_output.borrow().clone()
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
            let command = self.command.lock().unwrap();
            let code = command.exit.borrow();

            match &*code {
                (Some(r), _) => (Some(r.clone()), None),
                (_, Some(err)) => (None, Some(err.to_string())),
                _ => (None, None),
            }
        }
    }

    pub fn clean_output(&mut self) {
        self.output.borrow_mut().clear();
    }
}
