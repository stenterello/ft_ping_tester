use crate::app::utils::thread::Thread;
use crate::app::widgets::traits::viewer::OutputType;

pub trait ThreadLauncher {
    fn is_running(&self) -> bool;
    fn get_exit_status(&mut self) -> (Option<i32>, Option<String>) {
        self.thread_mut().get_exit()
    }
    fn thread_mut(&mut self) -> &mut Thread;
    fn thread(&self) -> &Thread;
    fn start_process(&mut self, args: Vec<String>) -> () {
        self.thread_mut().start(args);
    }
    fn take_output(&mut self, t: OutputType) -> Vec<String> {
        self.thread().take_output(t)
    }
    fn get_output(&self, t: OutputType) -> Vec<String> {
        self.thread().get_output(t)
    }

    fn clear_buffers(&mut self) -> () {
        self.thread_mut().clear_buffers()
    }
}
