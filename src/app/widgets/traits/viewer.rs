use ratatui::widgets::Widget;

use crate::app::utils::{enums::TextType, thread::Thread};

use super::thread_launcher::ThreadLauncher;

pub enum OutputType {
    Stdout,
    Stderr,
}

pub trait Viewer: ThreadLauncher {
    fn get_exit_status(&mut self) -> (Option<i32>, Option<String>) {
        self.thread_mut().get_exit()
    }
    fn thread_mut(&mut self) -> &mut Thread;
    fn thread(&self) -> &Thread;
    fn start_process(&mut self, args: Vec<String>) -> () {
        self.thread_mut().start(args);
    }
    fn take_output(&mut self, t: OutputType) -> Vec<String>;
    fn get_output(&self, t: OutputType) -> Vec<String> {
        self.thread().get_output(t)
    }
    fn set_text_to_display(&mut self, t: TextType) -> ();
    fn set_error_to_display(&mut self, t: TextType) -> ();
}
