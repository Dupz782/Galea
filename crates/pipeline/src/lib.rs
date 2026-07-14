use galea_core::{Frame, GaleaError};
use media::DecoderTraits;

pub struct Pipeline<D: DecoderTraits> {
    decoder: D,
}

impl<D: DecoderTraits> Pipeline<D> {
    pub fn new(decoder: D) -> Self {
        Self { decoder }
    }
    pub fn frame_at(&mut self, _time_ms: u64) -> Result<Option<Frame>, GaleaError> {
        self.decoder.next_frame()
    }
}
