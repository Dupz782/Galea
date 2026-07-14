use galea_core::{Frame, GaleaError};

pub struct Pose;

pub trait PoseTrackerTrait: Send {
    fn track(&mut self, frame: &Frame) -> Result<Pose, GaleaError>;
}
pub struct PoseTracker;
impl PoseTrackerTrait for PoseTracker {
    fn track(&mut self, frame: &Frame) -> Result<Pose, GaleaError> {
        Ok(Pose)
    }
}
