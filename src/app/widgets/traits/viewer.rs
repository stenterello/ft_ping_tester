use crate::app::utils::{enums::TextType};

pub enum OutputType {
    Stdout,
    Stderr,
}

pub trait Viewer {
    fn set_text_to_display(&mut self, t: TextType) -> ();
    fn set_error_to_display(&mut self, t: TextType) -> ();
}
