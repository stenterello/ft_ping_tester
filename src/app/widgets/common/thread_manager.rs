use crate::app::utils::thread::Thread;
use crate::app::widgets::traits::thread_launcher::ThreadLauncher;

#[derive (Debug)]
pub struct ThreadManager {
    thread: Thread,
}

impl ThreadManager {
    pub fn new(path: &str, name: &str) -> Self {
        Self {
            thread: Thread::new(path.into(), name.into())
        }
    }
}

impl ThreadLauncher for ThreadManager {
    fn is_running(&self) -> bool {
        self.thread.is_running()
    }

    fn thread_mut(&mut self) -> &mut Thread {
        &mut self.thread
    }

    fn thread(&self) -> &Thread {
        &self.thread
    }
}