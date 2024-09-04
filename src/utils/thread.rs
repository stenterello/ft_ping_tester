use crate::utils::subprocess::SubProcess;
use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct Thread {
    command: Arc<Mutex<SubProcess>>,
    rx: Receiver<String>,
    output: RefCell<Vec<String>>,
    name: String,
}

impl Thread {
    pub fn new(path: String) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            command: Arc::new(Mutex::new(SubProcess::new(path.clone(), tx))),
            rx,
            output: RefCell::new(Vec::default()),
            name: path,
        }
    }

    pub fn start(&mut self, args: Vec<String>) {
        let command = Arc::clone(&self.command);
        let handle = move || {
            let command = command.lock().unwrap();
            command.start(args);
        };

        let thread = thread::Builder::new().name(self.name.clone());

        thread.spawn(handle).unwrap();
    }

    pub fn get_output(&self) -> Vec<String> {
        match self.rx.try_recv() {
            Ok(received) => self.output.borrow_mut().push(received),
            _ => {}
        };
        self.output.borrow().clone()
    }
}
