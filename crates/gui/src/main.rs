use gui::run;
use media::{Decoder, DecoderTraits};
use pipeline::Pipeline;

fn main() {
    let decoder = Decoder::open("dummy.mp4").unwrap();
    let mut pipeline = Pipeline::new(decoder);
    let frame = pipeline.frame_at(0).unwrap().unwrap();
    run(frame);
}
