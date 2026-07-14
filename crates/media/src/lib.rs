use galea_core::{Frame, GaleaError};

pub trait DecoderTraits: Send + Sized {
    fn open(path: &str) -> Result<Self, GaleaError>
    where
        Self: Sized;
    fn next_frame(&mut self) -> Result<Option<Frame>, GaleaError>;
}

pub struct Decoder;
impl DecoderTraits for Decoder {
    fn open(_path: &str) -> Result<Self, GaleaError> {
        Ok(Self)
    }
    fn next_frame(&mut self) -> Result<Option<Frame>, GaleaError> {
        let (w, h) = (640, 420);
        let mut pixels = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let i = ((y * w + x) * 4) as usize;
                pixels[i] = (x * 255 / w) as u8;
                pixels[i + 1] = (y * 255 / h) as u8;
                pixels[i + 2] = 128;
                pixels[i + 3] = 255;
            }
        }
        Ok(Some(Frame {
            width: w,
            height: h,
            pixels,
            timestamp_ms: 0,
        }))
    }
}
