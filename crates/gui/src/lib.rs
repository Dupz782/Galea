use galea_core::Frame;
use slint;

pub fn frame_to_image(frame: &Frame) -> slint::Image {
    slint::Image::from_rgba8_premultiplied(slint::SharedPixelBuffer::clone_from_slice(
        &frame.pixels,
        frame.width,
        frame.height,
    ))
}

pub fn run(frame: Frame) {
    //slint file next time beh
    let _img = frame_to_image(&frame);
    println!(
        "Frame converted to Slint Image: {}x{}",
        frame.width, frame.height
    );
}
