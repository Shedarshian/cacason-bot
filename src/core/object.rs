use crate::core::segment::{PlacedSegment, Segment};

pub trait CanScore {

}

pub struct Object<'a> {
    segments: Vec<&'a PlacedSegment>
}

impl<'a> Object<'a> {
    pub fn create(seg: &'a PlacedSegment) -> Object<'a> {
        Object {
            segments: vec![seg]
        }
    }
    pub fn push(&mut self, seg: &'a PlacedSegment) {
        self.segments.push(seg)
    }
}