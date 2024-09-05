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
    output: RefCell<Vec<String>>,
    name: String,
    handle: Option<JoinHandle<Result<()>>>,
}

impl Thread {
    pub fn new(path: String) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            command: Arc::new(Mutex::new(SubProcess::new(path.clone(), tx))),
            rx,
            output: RefCell::new(Vec::default()),
            name: path,
            handle: None,
        }
    }

    pub fn start(&mut self, args: Vec<String>) {
        let command = Arc::clone(&self.command);
        let handle = move || -> Result<()> {
            let command = command.lock().unwrap();
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

    pub fn is_running(&self) -> bool {
        match &self.handle {
            Some(t) => !t.is_finished(),
            None => false,
        }
    }

    pub fn get_exit_status(&self) -> i32 {
        let command = self.command.lock().unwrap();
        let code = command.exit_status.borrow().code().unwrap();
        println!("Returning: {}", code);
        code
    }

    pub fn clean_output(&mut self) {
        self.output.borrow_mut().clear();
    }
}
