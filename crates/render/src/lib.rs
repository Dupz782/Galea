use galea_core::{Frame, GaleaError};

pub trait RendererTrait: Send {
    fn render(&self, frame: &Frame) -> Result<Frame, GaleaError>;
}

pub struct Renderer;
impl RendererTrait for Renderer {
    fn render(&self, frame: &Frame) -> Result<Frame, GaleaError> {
        Ok(frame.clone())
    }
}
